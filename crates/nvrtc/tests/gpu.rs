//! GPU tests (T2/T3) for `nvrtc`: runtime compilation to PTX/CUBIN, name expressions, build
//! log retrieval, and kernel launch via CUDA driver.

use cuda_bindings as cu;
use cuda_core::{Context, DeviceBuffer};
use nvrtc::{CompileOptions, Error, Program};
use std::ffi::{CString, c_void};

const SAXPY_SRC: &str = r#"
template<typename T>
__global__ void saxpy(T a, const T* x, const T* y, T* out, int n) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) {
        out[i] = a * x[i] + y[i];
    }
}
"#;

#[test]
#[ignore = "GPU"]
fn version_and_supported_archs() {
    let (maj, min) = nvrtc::version().expect("nvrtcVersion");
    assert!(maj >= 12, "unexpected NVRTC major version {maj}.{min}");
    let archs = nvrtc::supported_archs().expect("supported_archs");
    assert!(!archs.is_empty());
}

#[test]
#[ignore = "GPU"]
fn compile_ptx_inspect_and_launch_kernel() {
    let mut prog = Program::new(SAXPY_SRC, Some("saxpy.cu"), &[]).expect("create program");
    prog.add_name_expression("&saxpy<float>")
        .expect("add_name_expression");
    let opts = CompileOptions::new().std("c++17").line_info();
    let _log = prog.compile(&opts).expect("compile");

    let ptx_text = prog.ptx_string().expect("ptx_string");
    // Template __global__ instantiations emit `.weak .entry` rather than `.visible .entry`.
    assert!(ptx_text.contains(".entry"), "PTX missing .entry");
    let lowered = prog.lowered_name("&saxpy<float>").expect("lowered_name");
    assert!(
        ptx_text.contains(&lowered),
        "PTX missing lowered name {lowered}"
    );
    assert!(
        prog.cubin().expect("cubin query").is_none(),
        "virtual arch produces PTX only"
    );

    let ptx_bytes = prog.ptx().expect("ptx bytes");
    let ctx = Context::new(0).expect("CUDA device 0");
    let stream = ctx.default_stream();
    ctx.bind_to_thread().expect("bind");

    let n = 64usize;
    let a = 2.5f32;
    let x_host: Vec<f32> = (0..n).map(|i| i as f32).collect();
    let y_host: Vec<f32> = (0..n).map(|i| (i * 3) as f32).collect();
    let x_dev = DeviceBuffer::from_host(&stream, &x_host).expect("x_dev");
    let y_dev = DeviceBuffer::from_host(&stream, &y_host).expect("y_dev");
    let out_dev = DeviceBuffer::from_host(&stream, &vec![0.0f32; n]).expect("out_dev");

    let mut module: cu::CUmodule = std::ptr::null_mut();
    // SAFETY: `ptx_bytes` is NUL-terminated PTX from NVRTC and `module` is a valid out-pointer.
    let res = unsafe { cu::cuModuleLoadData(&mut module, ptx_bytes.as_ptr().cast()) };
    assert_eq!(
        res,
        cu::cudaError_enum_CUDA_SUCCESS,
        "cuModuleLoadData failed"
    );

    let lowered_c = CString::new(lowered).expect("cstr");
    let mut func: cu::CUfunction = std::ptr::null_mut();
    // SAFETY: `module` is loaded and `lowered_c` is NUL-terminated.
    let res = unsafe { cu::cuModuleGetFunction(&mut func, module, lowered_c.as_ptr()) };
    assert_eq!(
        res,
        cu::cudaError_enum_CUDA_SUCCESS,
        "cuModuleGetFunction failed"
    );

    let mut x_ptr = x_dev.cu_deviceptr();
    let mut y_ptr = y_dev.cu_deviceptr();
    let mut out_ptr = out_dev.cu_deviceptr();
    let mut n_i32 = n as i32;
    let mut a_val = a;
    let mut params: [*mut c_void; 5] = [
        (&raw mut a_val).cast(),
        (&raw mut x_ptr).cast(),
        (&raw mut y_ptr).cast(),
        (&raw mut out_ptr).cast(),
        (&raw mut n_i32).cast(),
    ];
    // SAFETY: `func` matches the 5-argument signature of `saxpy<float>`, and all device buffers
    // outlive the `stream.synchronize()` below.
    let res = unsafe {
        cu::cuLaunchKernel(
            func,
            1,
            1,
            1,
            n as u32,
            1,
            1,
            0,
            stream.cu_stream(),
            params.as_mut_ptr(),
            std::ptr::null_mut(),
        )
    };
    assert_eq!(
        res,
        cu::cudaError_enum_CUDA_SUCCESS,
        "cuLaunchKernel failed"
    );
    stream.synchronize().expect("sync");

    // SAFETY: `module` is no longer in use after `stream.synchronize()`.
    unsafe { cu::cuModuleUnload(module) };

    let got = out_dev.to_host_vec(&stream).expect("readback");
    for (i, &val) in got.iter().enumerate() {
        let expected = a * x_host[i] + y_host[i];
        assert!(
            (val - expected).abs() < 1e-5,
            "idx {i}: got {val}, expected {expected}"
        );
    }
}

#[test]
#[ignore = "GPU"]
fn compile_error_surfaces_build_log() {
    let mut prog = Program::new("__global__ void bad( { }", Some("bad.cu"), &[]).expect("create");
    let err = prog.compile(&CompileOptions::new()).expect_err("must fail");
    match err {
        Error::Compilation { log, .. } => {
            assert!(!log.is_empty(), "expected non-empty compiler log")
        }
        other => panic!("expected Error::Compilation, got {other:?}"),
    }
}

#[test]
#[ignore = "GPU"]
fn compile_cubin_for_supported_sm() {
    let archs = nvrtc::supported_archs().expect("archs");
    assert!(
        !archs.is_empty(),
        "expected at least one supported NVRTC arch"
    );
    let sm = format!("sm_{}", archs[0]);
    let mut prog =
        Program::new("extern \"C\" __global__ void noop() {}", None, &[]).expect("create");
    prog.compile(&CompileOptions::new().arch(&sm))
        .expect("compile sm");
    let cubin = prog
        .cubin()
        .expect("cubin")
        .expect("sm_XX should emit cubin");
    assert!(cubin.starts_with(b"\x7fELF"), "expected ELF cubin header");
}
