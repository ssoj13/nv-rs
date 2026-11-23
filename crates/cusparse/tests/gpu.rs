//! Numerical GPU tests (T3) for `cusparse`: run with
//! `cargo test -p cusparse -- --include-ignored --test-threads=1` on a CUDA GPU.

use cuda_core::{Context, DeviceBuffer, Stream};
use cusparse::{
    DataType, DiagType, DnMatDescr, DnVecDescr, Error, FillMode, Handle, IndexBase, IndexType, Op,
    Order, PointerMode, Scratch, ScratchError, SpGemm, SpGemmAlg, SpMatDescr, SpMmAlg, SpMvAlg, SpSm, SpSmAlg,
    SpSv, SpSvAlg, spmm, spmv, spmv_buffer_size,
};
use std::sync::Arc;
use std::time::Instant;

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
            budget,
        )
        .expect("work_estimation");
    // The budget path itself, independent of what cuSPARSE reports for this tiny product.
    let mut probe = Scratch::new(&stream);
    assert!(
        matches!(
            probe.reserve(&stream, 1, Some(0)),
            Err(ScratchError::Budget {
                requested: 1,
                available: 0
            })
        ),
        "Scratch must refuse a request above the budget"
    );
    if est_bytes == 0 {
        eprintln!(
            "note: SpGEMM work estimation reported 0 bytes; SpGEMM budget denial not exercised"
        );
    } else {
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

/// Bytes the caller scratch is pre-grown to, so a wrapper that ran an analysis phase in it
/// would reuse (and later see clobbered) this buffer.
const GROW: usize = 16 << 20;

/// Overwrites every byte the caller-owned `scratch` holds, in stream order on `stream`.
fn clobber(scratch: &mut Scratch, stream: &Stream) {
    let (ptr, cap) = scratch.reserve(stream, 0, Some(0)).expect("scratch pointer");
    assert!(cap > 0, "scratch was pre-grown, so it must own a buffer");
    // SAFETY: `ptr` is the start of `scratch`'s live allocation of `cap` bytes, used on `stream`.
    unsafe {
        cuda_core::memory::memset_d8_async(
            ptr,
            0xFF,
            cap,
            stream.cu_stream(),
        )
    }
    .expect("clobber scratch");
}

/// `A = [[1, 2], [0, 3]]`, `B = [[4, 0], [5, 6]]` (CSR, f32); `A * B = [[14, 12], [15, 18]]`.
struct GemmInputs {
    mat_a: SpMatDescr,
    mat_b: SpMatDescr,
    // Declared after the descriptors so they drop after them.
    _a: (DeviceBuffer<i32>, DeviceBuffer<i32>, DeviceBuffer<f32>),
    _b: (DeviceBuffer<i32>, DeviceBuffer<i32>, DeviceBuffer<f32>),
}

fn gemm_inputs(cuda: &Arc<Context>, stream: &Stream) -> GemmInputs {
    let a = (
        DeviceBuffer::from_host(stream, &[0i32, 2, 3]).expect("a_row"),
        DeviceBuffer::from_host(stream, &[0i32, 1, 1]).expect("a_col"),
        DeviceBuffer::from_host(stream, &[1.0f32, 2.0, 3.0]).expect("a_val"),
    );
    let b = (
        DeviceBuffer::from_host(stream, &[0i32, 1, 3]).expect("b_row"),
        DeviceBuffer::from_host(stream, &[0i32, 0, 1]).expect("b_col"),
        DeviceBuffer::from_host(stream, &[4.0f32, 5.0, 6.0]).expect("b_val"),
    );
    // SAFETY: the buffers move into the returned struct, which drops the descriptors first.
    let mat_a = unsafe { SpMatDescr::csr(cuda, 2, 2, 3, &a.0, &a.1, &a.2, IndexBase::Zero) }
        .expect("mat_a");
    // SAFETY: as above.
    let mat_b = unsafe { SpMatDescr::csr(cuda, 2, 2, 3, &b.0, &b.1, &b.2, IndexBase::Zero) }
        .expect("mat_b");
    GemmInputs {
        mat_a,
        mat_b,
        _a: a,
        _b: b,
    }
}

fn estimate_and_compute(
    spgemm: &mut SpGemm,
    handle: &Handle,
    stream: &Arc<Stream>,
    inputs: &GemmInputs,
    mat_c: &mut SpMatDescr,
    budget: usize,
) {
    spgemm
        .work_estimation(
            handle,
            stream,
            Op::NonTranspose,
            Op::NonTranspose,
            &1.0f32,
            &inputs.mat_a,
            &inputs.mat_b,
            &0.0f32,
            mat_c,
            DataType::F32,
            SpGemmAlg::Default,
            budget,
        )
        .expect("work_estimation");
    spgemm
        .compute(
            handle,
            stream,
            Op::NonTranspose,
            Op::NonTranspose,
            &1.0f32,
            &inputs.mat_a,
            &inputs.mat_b,
            &0.0f32,
            mat_c,
            DataType::F32,
            SpGemmAlg::Default,
            budget,
        )
        .expect("compute");
}

fn spgemm_copy(
    spgemm: &mut SpGemm,
    handle: &Handle,
    stream: &Stream,
    inputs: &GemmInputs,
    mat_c: &mut SpMatDescr,
) -> cusparse::Result<()> {
    spgemm.copy(
        handle,
        stream,
        Op::NonTranspose,
        Op::NonTranspose,
        &1.0f32,
        &inputs.mat_a,
        &inputs.mat_b,
        &0.0f32,
        mat_c,
        DataType::F32,
        SpGemmAlg::Default,
    )
}

fn empty_c(cuda: &Arc<Context>) -> SpMatDescr {
    SpMatDescr::csr_empty(
        cuda,
        2,
        2,
        IndexType::I32,
        IndexType::I32,
        IndexBase::Zero,
        DataType::F32,
    )
    .expect("mat_c")
}

/// P2: `cusparseSpSV_solve` reads the analysis buffer; reusing the caller scratch in between
/// must not corrupt it, and `solve` on another stream must be refused.
#[test]
#[ignore = "GPU: requires CUDA toolkit cusparse DLL and an NVIDIA GPU"]
fn spsv_solve_survives_reuse_of_caller_scratch() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let handle = Handle::new(&cuda).expect("cusparse handle");
    let mut scratch = Scratch::new(&stream);
    scratch
        .reserve(&stream, GROW, Some(GROW))
        .expect("pre-grow scratch");

    // L = [[2, 0, 0], [1, 3, 0], [0, 2, 4]], x = [4, 8, 12] -> y = [2, 2, 2]
    let row_off = DeviceBuffer::from_host(&stream, &[0i32, 1, 3, 5]).expect("row_off");
    let col_ind = DeviceBuffer::from_host(&stream, &[0i32, 0, 1, 1, 2]).expect("col_ind");
    let values = DeviceBuffer::from_host(&stream, &[2.0f32, 1.0, 3.0, 2.0, 4.0]).expect("values");
    let x = DeviceBuffer::from_host(&stream, &[4.0f32, 8.0, 12.0]).expect("x");
    let y = DeviceBuffer::<f32>::zeroed(&stream, 3).expect("y");
    // SAFETY: buffers outlive the descriptors and `synchronize`.
    let mut mat_l =
        unsafe { SpMatDescr::csr(&cuda, 3, 3, 5, &row_off, &col_ind, &values, IndexBase::Zero) }
            .expect("mat_l");
    mat_l.set_fill_mode(FillMode::Lower).expect("fill lower");
    mat_l
        .set_diag_type(DiagType::NonUnit)
        .expect("diag non-unit");
    // SAFETY: `x` outlives `vec_x`.
    let vec_x = unsafe { DnVecDescr::new(&cuda, &x) }.expect("vec_x");
    // SAFETY: `y` outlives `vec_y`.
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
        GROW,
    )
    .expect("spsv prepare");
    clobber(&mut scratch, &stream);

    let other = cuda.new_stream().expect("second stream");
    let refused = spsv.solve(
        &handle,
        &other,
        Op::NonTranspose,
        &1.0f32,
        &mat_l,
        &vec_x,
        &mut vec_y,
        DataType::F32,
        SpSvAlg::Default,
    );
    assert!(
        matches!(refused, Err(Error::InvalidInput(_))),
        "solve on another stream than prepare must be refused, got {refused:?}"
    );

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
}

/// P3: `cusparseSpGEMM_copy` reads buffer2 (and possibly buffer1); reusing the caller scratch
/// between `compute` and `copy` must not corrupt them.
#[test]
#[ignore = "GPU: requires CUDA toolkit cusparse DLL and an NVIDIA GPU"]
fn spgemm_copy_survives_reuse_of_caller_scratch() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let handle = Handle::new(&cuda).expect("cusparse handle");
    let mut scratch = Scratch::new(&stream);
    scratch
        .reserve(&stream, GROW, Some(GROW))
        .expect("pre-grow scratch");
    let inputs = gemm_inputs(&cuda, &stream);
    let mut mat_c = empty_c(&cuda);
    let mut spgemm = SpGemm::new(&cuda).expect("spgemm descr");

    estimate_and_compute(&mut spgemm, &handle, &stream, &inputs, &mut mat_c, GROW);
    clobber(&mut scratch, &stream);

    let (rows, _, nnz) = mat_c.query_size().expect("query_size");
    let c_row = DeviceBuffer::<i32>::zeroed(&stream, (rows + 1) as usize).expect("c_row");
    let c_col = DeviceBuffer::<i32>::zeroed(&stream, nnz as usize).expect("c_col");
    let c_val = DeviceBuffer::<f32>::zeroed(&stream, nnz as usize).expect("c_val");
    // SAFETY: `c_row`, `c_col`, `c_val` outlive `copy` and `synchronize`.
    unsafe { mat_c.set_csr_pointers(nnz, &c_row, &c_col, &c_val) }.expect("set_csr_pointers");
    spgemm_copy(&mut spgemm, &handle, &stream, &inputs, &mut mat_c).expect("copy");
    stream.synchronize().expect("sync");

    let vals = c_val.to_host_vec(&stream).expect("read c_val");
    let expected = [14.0f32, 12.0, 15.0, 18.0];
    let max_err = vals
        .iter()
        .zip(expected.iter())
        .map(|(u, v)| (u - v).abs())
        .fold(0.0f32, f32::max);
    assert!(max_err < 1e-5, "got={vals:?}, max_err={max_err}");
}

/// P4/P5: an unbound output descriptor is refused as an input and by `copy`; rebinding must use
/// the native nnz and the descriptor's index/value types.
#[test]
#[ignore = "GPU: requires CUDA toolkit cusparse DLL and an NVIDIA GPU"]
fn spgemm_output_binding_is_validated() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let handle = Handle::new(&cuda).expect("cusparse handle");
    let mut scratch = Scratch::new(&stream);
    let budget = 64 * 1024 * 1024;
    let inputs = gemm_inputs(&cuda, &stream);
    let mut mat_c = empty_c(&cuda);

    // P5: null-pointer descriptor used as SpMV input.
    let x = DeviceBuffer::from_host(&stream, &[1.0f32, 1.0]).expect("x");
    let y = DeviceBuffer::<f32>::zeroed(&stream, 2).expect("y");
    // SAFETY: `x` outlives `vec_x`.
    let vec_x = unsafe { DnVecDescr::new(&cuda, &x) }.expect("vec_x");
    // SAFETY: `y` outlives `vec_y`.
    let mut vec_y = unsafe { DnVecDescr::new(&cuda, &y) }.expect("vec_y");
    let unbound = spmv(
        &handle,
        &stream,
        Op::NonTranspose,
        &1.0f32,
        &mat_c,
        &vec_x,
        &0.0f32,
        &mut vec_y,
        DataType::F32,
        SpMvAlg::Default,
        &mut scratch,
        budget,
    );
    assert!(
        matches!(unbound, Err(Error::InvalidInput(_))),
        "unbound descriptor as SpMV input must be refused, got {unbound:?}"
    );

    let mut spgemm = SpGemm::new(&cuda).expect("spgemm descr");
    estimate_and_compute(&mut spgemm, &handle, &stream, &inputs, &mut mat_c, budget);
    let early = spgemm_copy(&mut spgemm, &handle, &stream, &inputs, &mut mat_c);
    assert!(
        matches!(early, Err(Error::InvalidInput(_))),
        "copy into an unbound descriptor must be refused, got {early:?}"
    );

    let (_, _, nnz) = mat_c.query_size().expect("query_size");
    assert_eq!(nnz, 4);
    let c_row = DeviceBuffer::<i32>::zeroed(&stream, 3).expect("c_row");
    let short_col = DeviceBuffer::<i32>::zeroed(&stream, 3).expect("short c_col");
    let short_val = DeviceBuffer::<f32>::zeroed(&stream, 3).expect("short c_val");
    // SAFETY: the call is expected to fail validation before binding anything.
    let wrong_nnz = unsafe { mat_c.set_csr_pointers(3, &c_row, &short_col, &short_val) };
    assert!(
        matches!(wrong_nnz, Err(Error::InvalidInput(_))),
        "nnz below the native nnz must be refused, got {wrong_nnz:?}"
    );

    let row64 = DeviceBuffer::<i64>::zeroed(&stream, 3).expect("c_row i64");
    let col64 = DeviceBuffer::<i64>::zeroed(&stream, 4).expect("c_col i64");
    let c_val = DeviceBuffer::<f32>::zeroed(&stream, 4).expect("c_val");
    // SAFETY: the call is expected to fail validation before binding anything.
    let wrong_type = unsafe { mat_c.set_csr_pointers(4, &row64, &col64, &c_val) };
    assert!(
        matches!(wrong_type, Err(Error::InvalidInput(_))),
        "index types differing from the descriptor must be refused, got {wrong_type:?}"
    );

    let c_col = DeviceBuffer::<i32>::zeroed(&stream, 4).expect("c_col");
    // SAFETY: `c_row`, `c_col`, `c_val` outlive `copy` and `synchronize`.
    unsafe { mat_c.set_csr_pointers(4, &c_row, &c_col, &c_val) }.expect("set_csr_pointers");
    spgemm_copy(&mut spgemm, &handle, &stream, &inputs, &mut mat_c).expect("copy");
    stream.synchronize().expect("sync");
    let vals = c_val.to_host_vec(&stream).expect("read c_val");
    let expected = [14.0f32, 12.0, 15.0, 18.0];
    let max_err = vals
        .iter()
        .zip(expected.iter())
        .map(|(u, v)| (u - v).abs())
        .fold(0.0f32, f32::max);
    assert!(max_err < 1e-5, "got={vals:?}, max_err={max_err}");
}

/// P7: no operation accepts device scalars, so device pointer mode is refused up front.
#[test]
#[ignore = "GPU: requires CUDA toolkit cusparse DLL and an NVIDIA GPU"]
fn set_pointer_mode_rejects_device() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let handle = Handle::new(&cuda).expect("cusparse handle");
    let res = handle.set_pointer_mode(PointerMode::Device);
    assert!(
        matches!(res, Err(Error::InvalidInput(_))),
        "device pointer mode must be refused, got {res:?}"
    );
    assert_eq!(handle.pointer_mode().expect("pointer mode"), PointerMode::Host);
    handle
        .set_pointer_mode(PointerMode::Host)
        .expect("host pointer mode");
}

/// P10 (Wave 1b identity rule): descriptors made through another `Arc<Context>` for the same CUDA
/// context (`Context::new` retains the device's primary context, so both `Arc`s name one
/// `CUcontext`) are accepted: contexts compare by value (`cuda_core_ext::same_context`), not by
/// `Arc`. A genuinely different context is still refused by `ensure_handle_context`.
#[test]
#[ignore = "GPU: requires CUDA toolkit cusparse DLL and an NVIDIA GPU"]
fn descriptors_from_another_arc_of_the_same_context_are_accepted() {
    let ctx_a = Context::new(0).expect("CUDA device 0 (a)");
    let ctx_b = Context::new(0).expect("CUDA device 0 (b)");
    let stream_a = ctx_a.default_stream();
    let stream_b = ctx_b.default_stream();
    let handle = Handle::new(&ctx_a).expect("cusparse handle");
    let mut scratch = Scratch::new(&stream_a);

    let row_off = DeviceBuffer::from_host(&stream_b, &[0i32, 1, 2]).expect("row_off");
    let col_ind = DeviceBuffer::from_host(&stream_b, &[0i32, 1]).expect("col_ind");
    let values = DeviceBuffer::from_host(&stream_b, &[1.0f32, 1.0]).expect("values");
    let x = DeviceBuffer::from_host(&stream_b, &[1.0f32, 2.0]).expect("x");
    let y = DeviceBuffer::<f32>::zeroed(&stream_b, 2).expect("y");
    // SAFETY: buffers outlive the descriptors.
    let mat_a =
        unsafe { SpMatDescr::csr(&ctx_b, 2, 2, 2, &row_off, &col_ind, &values, IndexBase::Zero) }
            .expect("mat_a");
    // SAFETY: `x` outlives `vec_x`.
    let vec_x = unsafe { DnVecDescr::new(&ctx_b, &x) }.expect("vec_x");
    // SAFETY: `y` outlives `vec_y`.
    let mut vec_y = unsafe { DnVecDescr::new(&ctx_b, &y) }.expect("vec_y");

    let res = spmv(
        &handle,
        &stream_a,
        Op::NonTranspose,
        &1.0f32,
        &mat_a,
        &vec_x,
        &0.0f32,
        &mut vec_y,
        DataType::F32,
        SpMvAlg::Default,
        &mut scratch,
        64 * 1024 * 1024,
    );
    assert!(
        cuda_core_ext::same_context(&ctx_a, &ctx_b),
        "two Context::new(0) share the primary context"
    );
    res.expect("descriptors of the same CUDA context through another Arc are accepted");
    stream_a.synchronize().expect("spmv completes");
}

/// Returns the context current on the calling thread (null if none).
fn current_ctx() -> cuda_core::sys::CUcontext {
    let mut cur: cuda_core::sys::CUcontext = std::ptr::null_mut();
    // SAFETY: `cur` is a valid out-pointer for `cuCtxGetCurrent`.
    let rc = unsafe { cuda_core::sys::cuCtxGetCurrent(&mut cur) };
    assert_eq!(rc, 0, "cuCtxGetCurrent failed: {rc}");
    cur
}

/// X2: the handle is `Send`; using it from a thread without a current context must bind the
/// handle's context. (With a single primary context the runtime may bind it implicitly, so this
/// fails before the fix only when the library does not touch the runtime in `cusparseSetStream`.)
#[test]
#[ignore = "GPU: requires CUDA toolkit cusparse DLL and an NVIDIA GPU"]
fn handle_set_stream_binds_context_on_another_thread() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let handle = Handle::new(&cuda).expect("cusparse handle");
    let want = cuda.cu_ctx() as usize;
    std::thread::spawn(move || {
        // SAFETY: detaching the current context of this thread is always valid.
        let rc = unsafe { cuda_core::sys::cuCtxSetCurrent(std::ptr::null_mut()) };
        assert_eq!(rc, 0, "cuCtxSetCurrent(null) failed: {rc}");
        handle.set_stream(&stream).expect("set_stream");
        assert_eq!(
            current_ctx() as usize,
            want,
            "set_stream must make the handle's context current"
        );
    })
    .join()
    .expect("worker thread");
}

/// P6 (decides it): `spmv` runs `cusparseSpMV_preprocess` on every call. Compares it with
/// a bare `cusparseSpMV` on the same descriptors and buffer; fails if per-call preprocessing
/// costs more than 50 %.
#[test]
#[ignore = "GPU: requires CUDA toolkit cusparse DLL and an NVIDIA GPU"]
fn spmv_per_call_preprocess_overhead_is_bounded() {
    const N: usize = 1 << 20;
    const ITERS: u32 = 50;
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let handle = Handle::new(&cuda).expect("cusparse handle");
    let mut scratch = Scratch::new(&stream);
    let budget = 1 << 30;

    // Tridiagonal [-1, 2, -1].
    let mut row_off_h = Vec::with_capacity(N + 1);
    let mut col_h = Vec::with_capacity(3 * N);
    let mut val_h = Vec::with_capacity(3 * N);
    row_off_h.push(0i32);
    for r in 0..N {
        for c in r.saturating_sub(1)..(r + 2).min(N) {
            col_h.push(c as i32);
            val_h.push(if c == r { 2.0f32 } else { -1.0 });
        }
        row_off_h.push(col_h.len() as i32);
    }
    let nnz = col_h.len() as i64;
    let row_off = DeviceBuffer::from_host(&stream, &row_off_h).expect("row_off");
    let col_ind = DeviceBuffer::from_host(&stream, &col_h).expect("col_ind");
    let values = DeviceBuffer::from_host(&stream, &val_h).expect("values");
    let x = DeviceBuffer::from_host(&stream, &vec![1.0f32; N]).expect("x");
    let y = DeviceBuffer::<f32>::zeroed(&stream, N).expect("y");
    let n = N as i64;
    // SAFETY: buffers outlive the descriptors and every `synchronize`.
    let mat_a = unsafe {
        SpMatDescr::csr(
            &cuda,
            n,
            n,
            nnz,
            &row_off,
            &col_ind,
            &values,
            IndexBase::Zero,
        )
    }
    .expect("mat_a");
    // SAFETY: `x` outlives `vec_x`.
    let vec_x = unsafe { DnVecDescr::new(&cuda, &x) }.expect("vec_x");
    // SAFETY: `y` outlives `vec_y`.
    let mut vec_y = unsafe { DnVecDescr::new(&cuda, &y) }.expect("vec_y");
    let (alpha, beta) = (1.0f32, 0.0f32);

    let mut wrapped_call = |vec_y: &mut DnVecDescr| {
        spmv(
            &handle,
            &stream,
            Op::NonTranspose,
            &alpha,
            &mat_a,
            &vec_x,
            &beta,
            vec_y,
            DataType::F32,
            SpMvAlg::Default,
            &mut scratch,
            budget,
        )
        .expect("spmv");
    };
    wrapped_call(&mut vec_y);
    stream.synchronize().expect("sync");
    let t = Instant::now();
    for _ in 0..ITERS {
        wrapped_call(&mut vec_y);
    }
    stream.synchronize().expect("sync");
    let wrapped = t.elapsed();

    let req = spmv_buffer_size(
        &handle,
        Op::NonTranspose,
        &alpha,
        &mat_a,
        &vec_x,
        &beta,
        &mut vec_y,
        DataType::F32,
        SpMvAlg::Default,
    )
    .expect("buffer size");
    let (ws, _) = scratch
        .reserve(&stream, req, Some(budget))
        .expect("workspace");
    let ws = ws as usize as *mut std::ffi::c_void;
    handle.set_stream(&stream).expect("set_stream");
    let raw_call = |vec_y: &mut DnVecDescr| {
        // SAFETY: descriptors and host scalars are live; `ws` holds `req` bytes on `stream`.
        let st = unsafe {
            cusparse::sys::cusparseSpMV(
                handle.raw(),
                cusparse::sys::cusparseOperation_t_CUSPARSE_OPERATION_NON_TRANSPOSE,
                std::ptr::from_ref(&alpha).cast::<std::ffi::c_void>(),
                mat_a.raw_const(),
                vec_x.raw_const(),
                std::ptr::from_ref(&beta).cast::<std::ffi::c_void>(),
                vec_y.raw(),
                cusparse::sys::cudaDataType_t_CUDA_R_32F,
                cusparse::sys::cusparseSpMVAlg_t_CUSPARSE_SPMV_ALG_DEFAULT,
                ws,
            )
        };
        assert_eq!(st, cusparse::sys::cusparseStatus_t_CUSPARSE_STATUS_SUCCESS);
    };
    raw_call(&mut vec_y);
    stream.synchronize().expect("sync");
    let t = Instant::now();
    for _ in 0..ITERS {
        raw_call(&mut vec_y);
    }
    stream.synchronize().expect("sync");
    let raw = t.elapsed();

    eprintln!("spmv wrapper {wrapped:?} vs bare cusparseSpMV {raw:?} for {ITERS} calls, nnz={nnz}");
    assert!(
        wrapped.as_secs_f64() <= 1.5 * raw.as_secs_f64(),
        "P6 confirmed: per-call preprocess makes spmv {:.2}x slower than bare SpMV",
        wrapped.as_secs_f64() / raw.as_secs_f64()
    );
}
