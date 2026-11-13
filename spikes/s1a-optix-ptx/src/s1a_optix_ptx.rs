//! Spike S1a (nv-rs PLAN.md §8): does OptiX accept a raygen program written in Rust and compiled
//! to PTX by cuda-oxide?
//!
//! Device side: `__raygen__ids` reads its launch index through `_optix_get_launch_index_{x,y}`
//! (inline PTX `call`, the same form `optix_device_impl.h` uses) and writes `index + 1` into a
//! buffer whose address comes from the OptiX launch params (`.const params`).
//! Host side: loads `nvoptix.dll` from the driver, queries the ABI-118 function table, builds a
//! module from the embedded PTX payload, a raygen + miss pipeline, a minimal SBT, launches 4x4
//! and checks every cell. Throwaway code: hand-rolled, no safe wrappers.

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

use cuda_core::embedded::{ArtifactPayloadKind, embedded_modules_from_current_exe};
use cuda_core::{CudaContext, DeviceBuffer};
use cuda_device::{ConstantMemory, constant, cuda_module, kernel, ptx_asm};
use std::ffi::{CStr, c_char, c_int, c_uint, c_void};
use std::ptr;

mod ox {
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/optix.rs"));
}

/// Launch params shared by host and device; layout must match on both sides.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Params {
    pub out: u64,
    pub width: u32,
    pub _pad: u32,
}

// SAFETY: plain integers; the all-zero pattern is a valid `Params`.
unsafe impl cuda_device::ConstantMemoryValue for Params {}

/// `OPTIX_SBT_RECORD_HEADER_SIZE` is a cast macro bindgen cannot evaluate.
const SBT_HEADER: usize = 32;

#[cuda_module]
mod kernels {
    use super::*;

    #[constant(export_name = "params")]
    static PARAMS: ConstantMemory<Params> = ConstantMemory::UNINIT;

    #[kernel]
    pub fn __raygen__ids() {
        let x: u32;
        let y: u32;
        // SAFETY: OptiX intrinsics valid in a raygen program; no memory side effects.
        unsafe {
            ptx_asm!("call (%0), _optix_get_launch_index_x, ();", out("=r") x);
            ptx_asm!("call (%0), _optix_get_launch_index_y, ();", out("=r") y);
        }
        let p = PARAMS.get();
        let i = y * p.width + x;
        // SAFETY: host allocates width*height u32 and launches exactly width x height.
        unsafe { *((p.out as *mut u32).add(i as usize)) = i + 1 };
    }

    #[kernel]
    pub fn __miss__noop() {}
}

type QueryFn = unsafe extern "C" fn(
    c_int,
    c_uint,
    *mut c_void,
    *mut *const c_void,
    *mut c_void,
    usize,
) -> ox::OptixResult;

fn find_nvoptix() -> Result<std::path::PathBuf, String> {
    let root = std::path::Path::new(r"C:\Windows\System32\DriverStore\FileRepository");
    let entries = std::fs::read_dir(root).map_err(|e| format!("read DriverStore: {e}"))?;
    for e in entries.flatten() {
        let name = e.file_name();
        if name.to_string_lossy().starts_with("nv_dispi.inf_amd64_") {
            let dll = e.path().join("nvoptix.dll");
            if dll.exists() {
                return Ok(dll);
            }
        }
    }
    Err("nvoptix.dll not found in DriverStore".into())
}

fn check(what: &str, r: ox::OptixResult, log: &[u8]) -> Result<(), String> {
    let text = CStr::from_bytes_until_nul(log).map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    if !text.trim().is_empty() {
        println!("[{what} log] {}", text.trim());
    }
    if r == ox::OptixResult_OPTIX_SUCCESS {
        Ok(())
    } else {
        Err(format!("{what} failed: OptixResult {r}"))
    }
}

unsafe extern "C" fn log_cb(level: c_uint, tag: *const c_char, msg: *const c_char, _: *mut c_void) {
    // SAFETY: OptiX passes valid NUL-terminated strings for the duration of the call.
    let (tag, msg) = unsafe { (CStr::from_ptr(tag), CStr::from_ptr(msg)) };
    println!("[optix {level} {}] {}", tag.to_string_lossy(), msg.to_string_lossy());
}

#[repr(C, align(16))]
struct Record([u8; SBT_HEADER]);

fn main() -> Result<(), String> {
    const W: u32 = 4;
    const H: u32 = 4;

    // --- PTX produced by cuda-oxide for this binary ---
    let modules = embedded_modules_from_current_exe().map_err(|e| format!("embedded modules: {e}"))?;
    let ptx = modules
        .iter()
        .find_map(|m| m.payload(ArtifactPayloadKind::Ptx))
        .ok_or("no PTX payload embedded")?;
    let ptx_text = String::from_utf8_lossy(ptx);
    println!("PTX: {} bytes; entries: {:?}", ptx.len(), ptx_text.lines().filter(|l| l.contains(".entry")).collect::<Vec<_>>());
    std::fs::write("target/s1a.ptx", ptx).map_err(|e| e.to_string())?;

    // --- CUDA ---
    let ctx = CudaContext::new(0).map_err(|e| format!("cuda ctx: {e:?}"))?;
    ctx.bind_to_thread().map_err(|e| format!("bind ctx: {e:?}"))?;
    let stream = ctx.default_stream();
    let out = DeviceBuffer::<u32>::zeroed(&stream, (W * H) as usize).map_err(|e| format!("{e:?}"))?;
    let params = Params { out: out.cu_deviceptr() as u64, width: W, _pad: 0 };
    // SAFETY: Params is repr(C) POD.
    let params_bytes = unsafe {
        std::slice::from_raw_parts(&params as *const Params as *const u8, size_of::<Params>())
    };
    let params_dev = DeviceBuffer::<u8>::from_host(&stream, params_bytes).map_err(|e| format!("{e:?}"))?;

    // --- OptiX function table ---
    let dll_path = find_nvoptix()?;
    println!("nvoptix: {}", dll_path.display());
    // SAFETY: loading the driver's OptiX library; symbol type matches optix_stubs.h.
    let lib = unsafe { libloading::Library::new(&dll_path) }.map_err(|e| e.to_string())?;
    let query: libloading::Symbol<QueryFn> =
        unsafe { lib.get(b"optixQueryFunctionTable\0") }.map_err(|e| e.to_string())?;
    let mut t = ox::OptixFunctionTable::default();
    let r = unsafe {
        query(
            ox::OPTIX_ABI_VERSION as c_int,
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            &mut t as *mut _ as *mut c_void,
            size_of::<ox::OptixFunctionTable>(),
        )
    };
    check("optixQueryFunctionTable", r, &[0])?;

    let mut log = vec![0u8; 8192];
    let mut log_len;
    unsafe {
        // Device context on the current CUDA context.
        let mut dctx: ox::OptixDeviceContext = ptr::null_mut();
        let opts = ox::OptixDeviceContextOptions {
            logCallbackFunction: Some(log_cb),
            logCallbackLevel: 4,
            ..Default::default()
        };
        check("deviceContextCreate", t.optixDeviceContextCreate.unwrap()(ctx.cu_ctx() as _, &opts, &mut dctx), &[0])?;

        let mco = ox::OptixModuleCompileOptions::default();
        // cuda-oxide ignores `#[constant(export_name)]` and mangles the symbol, so take the real
        // `.const` name from the PTX (finding for the fork patch list).
        let pname_str = ptx_text
            .lines()
            .find(|l| l.contains(".const") && l.contains("PARAMS"))
            .and_then(|l| l.split_whitespace().last())
            .and_then(|tok| tok.split('[').next())
            .ok_or("PARAMS .const symbol not found in PTX")?
            .to_owned();
        println!("launch params symbol: {pname_str}");
        let pname = std::ffi::CString::new(pname_str).map_err(|e| e.to_string())?;
        let pco = ox::OptixPipelineCompileOptions {
            traversableGraphFlags: ox::OptixTraversableGraphFlags_OPTIX_TRAVERSABLE_GRAPH_FLAG_ALLOW_SINGLE_GAS as u32,
            numPayloadValues: 0,
            numAttributeValues: 2,
            exceptionFlags: ox::OptixExceptionFlags_OPTIX_EXCEPTION_FLAG_NONE as u32,
            pipelineLaunchParamsVariableName: pname.as_ptr(),
            pipelineLaunchParamsSizeInBytes: size_of::<Params>(),
            usesPrimitiveTypeFlags: ox::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_TRIANGLE as u32,
            ..Default::default()
        };
        let mut module: ox::OptixModule = ptr::null_mut();
        log_len = log.len();
        let r = t.optixModuleCreate.unwrap()(
            dctx, &mco, &pco, ptx.as_ptr() as *const c_char, ptx.len(),
            log.as_mut_ptr() as *mut c_char, &mut log_len, &mut module,
        );
        check("optixModuleCreate", r, &log)?;
        println!("optixModuleCreate: OK");

        let mut descs = [ox::OptixProgramGroupDesc::default(), ox::OptixProgramGroupDesc::default()];
        descs[0].kind = ox::OptixProgramGroupKind_OPTIX_PROGRAM_GROUP_KIND_RAYGEN;
        descs[0].__bindgen_anon_1.raygen = ox::OptixProgramGroupSingleModule { module, entryFunctionName: c"__raygen__ids".as_ptr() };
        descs[1].kind = ox::OptixProgramGroupKind_OPTIX_PROGRAM_GROUP_KIND_MISS;
        descs[1].__bindgen_anon_1.miss = ox::OptixProgramGroupSingleModule { module, entryFunctionName: c"__miss__noop".as_ptr() };
        let pgo = ox::OptixProgramGroupOptions::default();
        let mut groups: [ox::OptixProgramGroup; 2] = [ptr::null_mut(); 2];
        log_len = log.len();
        let r = t.optixProgramGroupCreate.unwrap()(
            dctx, descs.as_ptr(), 2, &pgo, log.as_mut_ptr() as *mut c_char, &mut log_len, groups.as_mut_ptr(),
        );
        check("optixProgramGroupCreate", r, &log)?;

        let plo = ox::OptixPipelineLinkOptions { maxTraceDepth: 1, ..Default::default() };
        let mut pipeline: ox::OptixPipeline = ptr::null_mut();
        log_len = log.len();
        let r = t.optixPipelineCreate.unwrap()(
            dctx, &pco, &plo, groups.as_ptr(), 2, log.as_mut_ptr() as *mut c_char, &mut log_len, &mut pipeline,
        );
        check("optixPipelineCreate", r, &log)?;

        let mut rg = Record([0; 32]);
        let mut ms = Record([0; 32]);
        check("packHeader rg", t.optixSbtRecordPackHeader.unwrap()(groups[0], rg.0.as_mut_ptr() as *mut c_void), &[0])?;
        check("packHeader ms", t.optixSbtRecordPackHeader.unwrap()(groups[1], ms.0.as_mut_ptr() as *mut c_void), &[0])?;
        let rg_dev = DeviceBuffer::<u8>::from_host(&stream, &rg.0).map_err(|e| format!("{e:?}"))?;
        let ms_dev = DeviceBuffer::<u8>::from_host(&stream, &ms.0).map_err(|e| format!("{e:?}"))?;
        let sbt = ox::OptixShaderBindingTable {
            raygenRecord: rg_dev.cu_deviceptr() as _,
            missRecordBase: ms_dev.cu_deviceptr() as _,
            missRecordStrideInBytes: 32,
            missRecordCount: 1,
            ..Default::default()
        };
        stream.synchronize().map_err(|e| format!("{e:?}"))?;

        let r = t.optixLaunch.unwrap()(
            pipeline, stream.cu_stream() as _, params_dev.cu_deviceptr() as _, size_of::<Params>(), &sbt, W, H, 1,
        );
        check("optixLaunch", r, &[0])?;
        stream.synchronize().map_err(|e| format!("sync after launch: {e:?}"))?;

        let host = out.to_host_vec(&stream).map_err(|e| format!("{e:?}"))?;
        println!("result: {host:?}");
        let bad: Vec<_> = host.iter().enumerate().filter(|(i, v)| **v != *i as u32 + 1).collect();
        if !bad.is_empty() {
            return Err(format!("{} wrong cells: {bad:?}", bad.len()));
        }
        println!("S1a PASS: OptiX ran a raygen compiled from Rust by cuda-oxide ({W}x{H} launch indices correct)");

        t.optixPipelineDestroy.unwrap()(pipeline);
        t.optixProgramGroupDestroy.unwrap()(groups[0]);
        t.optixProgramGroupDestroy.unwrap()(groups[1]);
        t.optixModuleDestroy.unwrap()(module);
        t.optixDeviceContextDestroy.unwrap()(dctx);
    }
    Ok(())
}
