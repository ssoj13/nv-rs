//! Numerical GPU tests (T3) for `cusparse`: run with
//! `cargo test -p cusparse -- --include-ignored --test-threads=1` on a CUDA GPU.

use cuda_core::{Context, DeviceBuffer};
use cusparse::{
    DataType, DiagType, DnMatDescr, DnVecDescr, Error, FillMode, Handle, IndexBase, IndexType, Op,
    Order, Scratch, SpGemm, SpGemmAlg, SpMatDescr, SpMmAlg, SpMvAlg, SpSm, SpSmAlg, SpSv, SpSvAlg,
    spmm, spmv,
};

#[test]
#[ignore = "GPU: requires CUDA toolkit cusparse DLL and an NVIDIA GPU"]
fn spmv_csr_f32_matches_reference() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let handle = Handle::new(&cuda).expect("cusparse handle");
    let mut scratch = Scratch::new(&stream);

    // A = [[1, 0, 2], [0, 3, 0], [4, 0, 5]]
    // x = [1, 2, 3], y0 = [10, 20, 30]
    // y = 2 * A * x + 0.5 * y0 = [2*7 + 5, 2*6 + 10, 2*19 + 15] = [19, 22, 53]
    let row_off = DeviceBuffer::from_host(&stream, &[0i32, 2, 3, 5]).expect("row_off");
    let col_ind = DeviceBuffer::from_host(&stream, &[0i32, 2, 1, 0, 2]).expect("col_ind");
    let values = DeviceBuffer::from_host(&stream, &[1.0f32, 2.0, 3.0, 4.0, 5.0]).expect("values");
    let x = DeviceBuffer::from_host(&stream, &[1.0f32, 2.0, 3.0]).expect("x");
    let y = DeviceBuffer::from_host(&stream, &[10.0f32, 20.0, 30.0]).expect("y");

    // SAFETY: all device buffers stay alive across `spmv` and `stream.synchronize()`.
    let mat_a =
        unsafe { SpMatDescr::csr(&cuda, 3, 3, 5, &row_off, &col_ind, &values, IndexBase::Zero) }
            .expect("mat_a");
    // SAFETY: `x` stays alive across `spmv` and `stream.synchronize()`.
    let vec_x = unsafe { DnVecDescr::new(&cuda, &x) }.expect("vec_x");
    // SAFETY: `y` stays alive across `spmv` and `stream.synchronize()`.
    let mut vec_y = unsafe { DnVecDescr::new(&cuda, &y) }.expect("vec_y");

    spmv(
        &handle,
        &stream,
        Op::NonTranspose,
        &2.0f32,
        &mat_a,
        &vec_x,
        &0.5f32,
        &mut vec_y,
        DataType::F32,
        SpMvAlg::Default,
        &mut scratch,
        64 * 1024 * 1024,
    )
    .expect("spmv");
    stream.synchronize().expect("sync");

    let got = y.to_host_vec(&stream).expect("read y");
    let expected = [19.0f32, 22.0, 53.0];
    let max_err = got
        .iter()
        .zip(expected.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max);
    assert!(
        max_err < 1e-5,
        "got={got:?}, expected={expected:?}, max_err={max_err}"
    );
}

#[test]
#[ignore = "GPU: requires CUDA toolkit cusparse DLL and an NVIDIA GPU"]
fn spmm_csr_f32_matches_reference() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let handle = Handle::new(&cuda).expect("cusparse handle");
    let mut scratch = Scratch::new(&stream);

    // A (3x3) = [[1, 0, 2], [0, 3, 0], [4, 0, 5]]
    // B (3x2, col-major) = [[1, 4], [2, 5], [3, 6]] -> [1, 2, 3, 4, 5, 6]
    // A * B = [[7, 16], [6, 15], [19, 46]] -> col-major [7, 6, 19, 16, 15, 46]
    let row_off = DeviceBuffer::from_host(&stream, &[0i32, 2, 3, 5]).expect("row_off");
    let col_ind = DeviceBuffer::from_host(&stream, &[0i32, 2, 1, 0, 2]).expect("col_ind");
    let values = DeviceBuffer::from_host(&stream, &[1.0f32, 2.0, 3.0, 4.0, 5.0]).expect("values");
    let b = DeviceBuffer::from_host(&stream, &[1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0]).expect("b");
    let c = DeviceBuffer::zeroed(&stream, 6).expect("c");

    // SAFETY: buffers outlive `spmm` and `synchronize`.
    let mat_a =
        unsafe { SpMatDescr::csr(&cuda, 3, 3, 5, &row_off, &col_ind, &values, IndexBase::Zero) }
            .expect("mat_a");
    // SAFETY: `b` outlives `spmm`.
    let mat_b = unsafe { DnMatDescr::new(&cuda, 3, 2, 3, &b, Order::Col) }.expect("mat_b");
    // SAFETY: `c` outlives `spmm`.
    let mut mat_c = unsafe { DnMatDescr::new(&cuda, 3, 2, 3, &c, Order::Col) }.expect("mat_c");

    spmm(
        &handle,
        &stream,
        Op::NonTranspose,
        Op::NonTranspose,
        &1.0f32,
        &mat_a,
        &mat_b,
        &0.0f32,
        &mut mat_c,
        DataType::F32,
        SpMmAlg::Default,
        &mut scratch,
        64 * 1024 * 1024,
    )
    .expect("spmm");
    stream.synchronize().expect("sync");

    let got = c.to_host_vec(&stream).expect("read c");
    let expected = [7.0f32, 6.0, 19.0, 16.0, 15.0, 46.0];
    let max_err = got
        .iter()
        .zip(expected.iter())
        .map(|(x, y)| (x - y).abs())
        .fold(0.0f32, f32::max);
    assert!(
        max_err < 1e-5,
        "got={got:?}, expected={expected:?}, max_err={max_err}"
    );
}

#[test]
#[ignore = "GPU: requires CUDA toolkit cusparse DLL and an NVIDIA GPU"]
fn spgemm_csr_f32_matches_reference_and_enforces_budget() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let handle = Handle::new(&cuda).expect("cusparse handle");
    let mut scratch = Scratch::new(&stream);
    let mut spgemm = SpGemm::new(&cuda).expect("spgemm descr");

    // A = [[1, 2], [0, 3]], B = [[4, 0], [5, 6]] -> C = [[14, 12], [15, 18]]
    let a_row = DeviceBuffer::from_host(&stream, &[0i32, 2, 3]).expect("a_row");
    let a_col = DeviceBuffer::from_host(&stream, &[0i32, 1, 1]).expect("a_col");
    let a_val = DeviceBuffer::from_host(&stream, &[1.0f32, 2.0, 3.0]).expect("a_val");
    let b_row = DeviceBuffer::from_host(&stream, &[0i32, 1, 3]).expect("b_row");
    let b_col = DeviceBuffer::from_host(&stream, &[0i32, 0, 1]).expect("b_col");
    let b_val = DeviceBuffer::from_host(&stream, &[4.0f32, 5.0, 6.0]).expect("b_val");

    // SAFETY: buffers outlive `spgemm` phases and `synchronize`.
    let mat_a = unsafe { SpMatDescr::csr(&cuda, 2, 2, 3, &a_row, &a_col, &a_val, IndexBase::Zero) }
        .expect("mat_a");
    // SAFETY: buffers outlive `spgemm` phases.
    let mat_b = unsafe { SpMatDescr::csr(&cuda, 2, 2, 3, &b_row, &b_col, &b_val, IndexBase::Zero) }
        .expect("mat_b");
    let mut mat_c = SpMatDescr::csr_empty(
        &cuda,
        2,
        2,
        IndexType::I32,
        IndexType::I32,
        IndexBase::Zero,
        DataType::F32,
    )
    .expect("mat_c");

    let budget = 64 * 1024 * 1024;
    let est_bytes = spgemm
        .work_estimation(
            &handle,
            &stream,
            Op::NonTranspose,
            Op::NonTranspose,
            &1.0f32,
            &mat_a,
            &mat_b,
            &0.0f32,
            &mut mat_c,
            DataType::F32,
            SpGemmAlg::Default,
            &mut scratch,
            budget,
        )
        .expect("work_estimation");
    if est_bytes > 0 {
        let mut tiny_scratch = Scratch::new(&stream);
        let denied = spgemm.work_estimation(
            &handle,
            &stream,
            Op::NonTranspose,
            Op::NonTranspose,
            &1.0f32,
            &mat_a,
            &mat_b,
            &0.0f32,
            &mut mat_c,
            DataType::F32,
            SpGemmAlg::Default,
            &mut tiny_scratch,
            0,
        );
        assert!(
            matches!(denied, Err(Error::Budget { .. })),
            "expected Budget error, got {denied:?}"
        );
    }

    spgemm
        .compute(
            &handle,
            &stream,
            Op::NonTranspose,
            Op::NonTranspose,
            &1.0f32,
            &mat_a,
            &mat_b,
            &0.0f32,
            &mut mat_c,
            DataType::F32,
            SpGemmAlg::Default,
            &mut scratch,
            budget,
        )
        .expect("compute");

    let (rows, cols, nnz) = mat_c.query_size().expect("query_size");
    assert_eq!((rows, cols, nnz), (2, 2, 4));

    let c_row = DeviceBuffer::<i32>::zeroed(&stream, (rows + 1) as usize).expect("c_row");
    let c_col = DeviceBuffer::<i32>::zeroed(&stream, nnz as usize).expect("c_col");
    let c_val = DeviceBuffer::<f32>::zeroed(&stream, nnz as usize).expect("c_val");
    // SAFETY: `c_row`, `c_col`, `c_val` stay alive across `copy` and `synchronize`.
    unsafe { mat_c.set_csr_pointers(nnz, &c_row, &c_col, &c_val) }.expect("set_csr_pointers");

    spgemm
        .copy(
            &handle,
            &stream,
            Op::NonTranspose,
            Op::NonTranspose,
            &1.0f32,
            &mat_a,
            &mat_b,
            &0.0f32,
            &mut mat_c,
            DataType::F32,
            SpGemmAlg::Default,
        )
        .expect("copy");
    stream.synchronize().expect("sync");

    let vals = c_val.to_host_vec(&stream).expect("read c_val");
    let expected = [14.0f32, 12.0, 15.0, 18.0];
    let max_err = vals
        .iter()
        .zip(expected.iter())
        .map(|(x, y)| (x - y).abs())
        .fold(0.0f32, f32::max);
    assert!(
        max_err < 1e-5,
        "got={vals:?}, expected={expected:?}, max_err={max_err}"
    );
}

#[test]
#[ignore = "GPU: requires CUDA toolkit cusparse DLL and an NVIDIA GPU"]
fn spsv_and_spsm_lower_triangular_solve() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let handle = Handle::new(&cuda).expect("cusparse handle");
    let mut scratch = Scratch::new(&stream);

    // L = [[2, 0, 0], [1, 3, 0], [0, 2, 4]]
    // x = [4, 8, 12] -> L * y = x => y = [2, 2, 2]
    let row_off = DeviceBuffer::from_host(&stream, &[0i32, 1, 3, 5]).expect("row_off");
    let col_ind = DeviceBuffer::from_host(&stream, &[0i32, 0, 1, 1, 2]).expect("col_ind");
    let values = DeviceBuffer::from_host(&stream, &[2.0f32, 1.0, 3.0, 2.0, 4.0]).expect("values");
    let x = DeviceBuffer::from_host(&stream, &[4.0f32, 8.0, 12.0]).expect("x");
    let y = DeviceBuffer::<f32>::zeroed(&stream, 3).expect("y");

    // SAFETY: buffers outlive `SpSv` and `synchronize`.
    let mut mat_l =
        unsafe { SpMatDescr::csr(&cuda, 3, 3, 5, &row_off, &col_ind, &values, IndexBase::Zero) }
            .expect("mat_l");
    mat_l.set_fill_mode(FillMode::Lower).expect("fill lower");
    mat_l
        .set_diag_type(DiagType::NonUnit)
        .expect("diag non-unit");
    // SAFETY: `x` outlives `SpSv`.
    let vec_x = unsafe { DnVecDescr::new(&cuda, &x) }.expect("vec_x");
    // SAFETY: `y` outlives `SpSv`.
    let mut vec_y = unsafe { DnVecDescr::new(&cuda, &y) }.expect("vec_y");

    let mut spsv = SpSv::new(&cuda).expect("spsv");
    spsv.prepare(
        &handle,
        &stream,
        Op::NonTranspose,
        &1.0f32,
        &mat_l,
        &vec_x,
        &mut vec_y,
        DataType::F32,
        SpSvAlg::Default,
        &mut scratch,
        64 * 1024 * 1024,
    )
    .expect("spsv prepare");
    spsv.solve(
        &handle,
        &stream,
        Op::NonTranspose,
        &1.0f32,
        &mat_l,
        &vec_x,
        &mut vec_y,
        DataType::F32,
        SpSvAlg::Default,
    )
    .expect("spsv solve");
    stream.synchronize().expect("sync");

    let got = y.to_host_vec(&stream).expect("read y");
    let max_err = got
        .iter()
        .map(|v| (*v - 2.0f32).abs())
        .fold(0.0f32, f32::max);
    assert!(max_err < 1e-5, "SpSV got={got:?}, max_err={max_err}");

    // SpSM with 2 RHS columns: B = [[4, 2], [8, 4], [12, 6]] -> C = [[2, 1], [2, 1], [2, 1]]
    let b = DeviceBuffer::from_host(&stream, &[4.0f32, 8.0, 12.0, 2.0, 4.0, 6.0]).expect("b");
    let c = DeviceBuffer::<f32>::zeroed(&stream, 6).expect("c");
    // SAFETY: `b` outlives `SpSm`.
    let mat_b = unsafe { DnMatDescr::new(&cuda, 3, 2, 3, &b, Order::Col) }.expect("mat_b");
    // SAFETY: `c` outlives `SpSm`.
    let mut mat_c = unsafe { DnMatDescr::new(&cuda, 3, 2, 3, &c, Order::Col) }.expect("mat_c");

    let mut spsm = SpSm::new(&cuda).expect("spsm");
    spsm.prepare(
        &handle,
        &stream,
        Op::NonTranspose,
        Op::NonTranspose,
        &1.0f32,
        &mat_l,
        &mat_b,
        &mut mat_c,
        DataType::F32,
        SpSmAlg::Default,
        &mut scratch,
        64 * 1024 * 1024,
    )
    .expect("spsm prepare");
    spsm.solve(
        &handle,
        &stream,
        Op::NonTranspose,
        Op::NonTranspose,
        &1.0f32,
        &mat_l,
        &mat_b,
        &mut mat_c,
        DataType::F32,
        SpSmAlg::Default,
    )
    .expect("spsm solve");
    stream.synchronize().expect("sync");

    let got_c = c.to_host_vec(&stream).expect("read c");
    let expected_c = [2.0f32, 2.0, 2.0, 1.0, 1.0, 1.0];
    let max_err_c = got_c
        .iter()
        .zip(expected_c.iter())
        .map(|(u, v)| (u - v).abs())
        .fold(0.0f32, f32::max);
    assert!(max_err_c < 1e-5, "SpSM got={got_c:?}, max_err={max_err_c}");
}
