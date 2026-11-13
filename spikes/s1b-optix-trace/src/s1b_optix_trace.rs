//! Spike S1b (nv-rs PLAN.md §8): can a raygen written in Rust call `optixTrace`?
//!
//! Device side (cuda-oxide, fork branch `feat/optix-device`): `__raygen__mask` shoots one
//! orthographic ray per pixel along +z through `_optix_trace_typed_32`, the same 32-output /
//! 49-input inline-PTX `call` that `optix_device_impl.h` emits. `__closesthit__hit` and
//! `__miss__none` write payload 0 through `_optix_set_payload`. Launch params are found by the
//! name `params`, which needs the fork's `#[constant(export_name)]` fix.
//! Host side: builds a one-triangle GAS, a raygen + miss + hit-group pipeline and SBT, launches
//! 16x16 and compares the hit mask with the analytic one. Throwaway code: hand-rolled, no safe
//! wrappers (those are M4).

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

use cuda_core::embedded::{ArtifactPayloadKind, embedded_modules_from_current_exe};
use cuda_core::{Context, DeviceBuffer};
use cuda_device::{ConstantMemory, constant, cuda_module, kernel, ptx_asm};
use std::ffi::{CStr, c_char, c_int, c_uint, c_void};
use std::ptr;

mod ox {
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/optix.rs"));
}

const W: u32 = 16;
const H: u32 = 16;
/// Triangle legs: (0,0,0), (LEG,0,0), (0,LEG,0). Pixel centres sit at (x+0.5)/W, so the hit test
/// `u + v < LEG` is at least 0.03 away from every centre: no pixel lands on an edge.
const LEG: f32 = 1.03;
/// Payload value before the trace; seeing it afterwards means neither program ran.
const SENTINEL: u32 = 7;

/// Launch params shared by host and device; layout must match on both sides.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Params {
    pub out: u64,
    pub handle: u64,
    pub width: u32,
    pub height: u32,
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
    pub fn __raygen__mask() {
        let x: u32;
        let y: u32;
        // SAFETY: OptiX intrinsics valid in a raygen program; no memory side effects.
        unsafe {
            ptx_asm!("call (%0), _optix_get_launch_index_x, ();", out("=r") x);
            ptx_asm!("call (%0), _optix_get_launch_index_y, ();", out("=r") y);
        }
        let p = PARAMS.get();
        let ox = (x as f32 + 0.5) / p.width as f32;
        let oy = (y as f32 + 0.5) / p.height as f32;
        let oz = -1.0f32;
        let handle = p.handle;
        let p0: u32;
        let _p1: u32;
        let _p2: u32;
        let _p3: u32;
        let _p4: u32;
        let _p5: u32;
        let _p6: u32;
        let _p7: u32;
        let _p8: u32;
        let _p9: u32;
        let _p10: u32;
        let _p11: u32;
        let _p12: u32;
        let _p13: u32;
        let _p14: u32;
        let _p15: u32;
        let _p16: u32;
        let _p17: u32;
        let _p18: u32;
        let _p19: u32;
        let _p20: u32;
        let _p21: u32;
        let _p22: u32;
        let _p23: u32;
        let _p24: u32;
        let _p25: u32;
        let _p26: u32;
        let _p27: u32;
        let _p28: u32;
        let _p29: u32;
        let _p30: u32;
        let _p31: u32;
        // SAFETY: `optixTrace` with the default payload type, one payload register, a single
        // GAS handle from the launch params and SBT stride/offset matching the host SBT.
        unsafe {
            ptx_asm!(
                "call (%0,%1,%2,%3,%4,%5,%6,%7,%8,%9,%10,%11,%12,%13,%14,%15,%16,%17,%18,%19,%20,%21,%22,%23,%24,%25,%26,%27,%28,%29,%30,%31), _optix_trace_typed_32, (%32,%33,%34,%35,%36,%37,%38,%39,%40,%41,%42,%43,%44,%45,%46,%47,%48,%49,%50,%51,%52,%53,%54,%55,%56,%57,%58,%59,%60,%61,%62,%63,%64,%65,%66,%67,%68,%69,%70,%71,%72,%73,%74,%75,%76,%77,%78,%79,%80);",
                out("=r") p0,
                out("=r") _p1,
                out("=r") _p2,
                out("=r") _p3,
                out("=r") _p4,
                out("=r") _p5,
                out("=r") _p6,
                out("=r") _p7,
                out("=r") _p8,
                out("=r") _p9,
                out("=r") _p10,
                out("=r") _p11,
                out("=r") _p12,
                out("=r") _p13,
                out("=r") _p14,
                out("=r") _p15,
                out("=r") _p16,
                out("=r") _p17,
                out("=r") _p18,
                out("=r") _p19,
                out("=r") _p20,
                out("=r") _p21,
                out("=r") _p22,
                out("=r") _p23,
                out("=r") _p24,
                out("=r") _p25,
                out("=r") _p26,
                out("=r") _p27,
                out("=r") _p28,
                out("=r") _p29,
                out("=r") _p30,
                out("=r") _p31,
                in("r") 0u32, // OPTIX_PAYLOAD_TYPE_DEFAULT
                in("l") handle,
                in("f") ox,
                in("f") oy,
                in("f") oz,
                in("f") 0.0f32,
                in("f") 0.0f32,
                in("f") 1.0f32,
                in("f") 0.0f32, // tmin
                in("f") 1.0e16f32, // tmax
                in("f") 0.0f32, // ray time
                in("r") 0xFFu32, // visibility mask
                in("r") 0u32, // ray flags
                in("r") 0u32, // SBT offset
                in("r") 1u32, // SBT stride
                in("r") 0u32, // miss index
                in("r") 1u32, // payload count
                in("r") SENTINEL,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
                in("r") 0u32,
            );
        }
        // SAFETY: host allocates width*height u32 and launches exactly width x height.
        unsafe { *((p.out as *mut u32).add((y * p.width + x) as usize)) = p0 };
    }

    #[kernel]
    pub fn __closesthit__hit() {
        // SAFETY: payload 0 exists (numPayloadValues = 1).
        unsafe { ptx_asm!("call _optix_set_payload, (%0, %1);", in("r") 0u32, in("r") 1u32) };
    }

    #[kernel]
    pub fn __miss__none() {
        // SAFETY: as above.
        unsafe { ptx_asm!("call _optix_set_payload, (%0, %1);", in("r") 0u32, in("r") 0u32) };
    }
}

type QueryFn = unsafe extern "C" fn(c_int, c_uint, *mut c_void, *mut *const c_void, *mut c_void, usize) -> ox::OptixResult;

fn find_nvoptix() -> Result<std::path::PathBuf, String> {
    let root = std::path::Path::new(r"C:\Windows\System32\DriverStore\FileRepository");
    for e in std::fs::read_dir(root).map_err(|e| format!("read DriverStore: {e}"))?.flatten() {
        if e.file_name().to_string_lossy().starts_with("nv_dispi.inf_amd64_") {
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
    if r == ox::OptixResult_OPTIX_SUCCESS { Ok(()) } else { Err(format!("{what} failed: OptixResult {r}")) }
}

unsafe extern "C" fn log_cb(level: c_uint, tag: *const c_char, msg: *const c_char, _: *mut c_void) {
    // SAFETY: OptiX passes valid NUL-terminated strings for the duration of the call.
    let (tag, msg) = unsafe { (CStr::from_ptr(tag), CStr::from_ptr(msg)) };
    println!("[optix {level} {}] {}", tag.to_string_lossy(), msg.to_string_lossy());
}

#[repr(C, align(16))]
struct Record([u8; SBT_HEADER]);

fn e<E: std::fmt::Debug>(err: E) -> String {
    format!("{err:?}")
}

fn main() -> Result<(), String> {
    let modules = embedded_modules_from_current_exe().map_err(e)?;
    let ptx = modules.iter().find_map(|m| m.payload(ArtifactPayloadKind::Ptx)).ok_or("no PTX payload embedded")?;
    std::fs::write("target/s1b.ptx", ptx).map_err(e)?;
    let ptx_text = String::from_utf8_lossy(ptx);
    if !ptx_text.lines().any(|l| l.contains(".const") && l.contains(" params")) {
        return Err("PTX has no `.const params`: #[constant(export_name)] was not honoured".into());
    }

    let ctx = Context::new(0).map_err(e)?;
    ctx.bind_to_thread().map_err(e)?;
    let stream = ctx.default_stream();

    let dll = find_nvoptix()?;
    // SAFETY: loading the driver's OptiX library; symbol type matches optix_stubs.h.
    let lib = unsafe { libloading::Library::new(&dll) }.map_err(e)?;
    // SAFETY: as above.
    let query: libloading::Symbol<QueryFn> = unsafe { lib.get(b"optixQueryFunctionTable\0") }.map_err(e)?;
    let mut t = ox::OptixFunctionTable::default();
    // SAFETY: table pointer and size match the header the bindings came from.
    let r = unsafe {
        query(ox::OPTIX_ABI_VERSION as c_int, 0, ptr::null_mut(), ptr::null_mut(), (&raw mut t).cast(), size_of::<ox::OptixFunctionTable>())
    };
    check("optixQueryFunctionTable", r, &[0])?;

    let verts: [f32; 9] = [0.0, 0.0, 0.0, LEG, 0.0, 0.0, 0.0, LEG, 0.0];
    let verts_dev = DeviceBuffer::<f32>::from_host(&stream, &verts).map_err(e)?;
    let out = DeviceBuffer::<u32>::zeroed(&stream, (W * H) as usize).map_err(e)?;

    let mut log = vec![0u8; 8192];
    let mut log_len;
    // SAFETY: the whole block is the raw OptiX host API driven exactly as the SDK samples do;
    // every pointer passed lives until the call returns, device buffers outlive the launch
    // (synchronized before they drop).
    unsafe {
        let mut dctx: ox::OptixDeviceContext = ptr::null_mut();
        let opts = ox::OptixDeviceContextOptions { logCallbackFunction: Some(log_cb), logCallbackLevel: 4, ..Default::default() };
        check("deviceContextCreate", t.optixDeviceContextCreate.unwrap()(ctx.cu_ctx() as _, &opts, &mut dctx), &[0])?;

        // --- GAS: one triangle ---
        let vptr = verts_dev.cu_deviceptr() as ox::CUdeviceptr;
        let flags = [ox::OptixGeometryFlags_OPTIX_GEOMETRY_FLAG_NONE as u32];
        let mut input = ox::OptixBuildInput { type_: ox::OptixBuildInputType_OPTIX_BUILD_INPUT_TYPE_TRIANGLES, ..Default::default() };
        input.__bindgen_anon_1.triangleArray = ox::OptixBuildInputTriangleArray {
            vertexBuffers: &vptr,
            numVertices: 3,
            vertexFormat: ox::OptixVertexFormat_OPTIX_VERTEX_FORMAT_FLOAT3,
            vertexStrideInBytes: 12,
            flags: flags.as_ptr(),
            numSbtRecords: 1,
            ..Default::default()
        };
        let accel = ox::OptixAccelBuildOptions {
            buildFlags: ox::OptixBuildFlags_OPTIX_BUILD_FLAG_NONE as u32,
            operation: ox::OptixBuildOperation_OPTIX_BUILD_OPERATION_BUILD,
            ..Default::default()
        };
        let mut sizes = ox::OptixAccelBufferSizes::default();
        check("accelComputeMemoryUsage", t.optixAccelComputeMemoryUsage.unwrap()(dctx, &accel, &input, 1, &mut sizes), &[0])?;
        let temp = DeviceBuffer::<u8>::zeroed(&stream, sizes.tempSizeInBytes.max(1)).map_err(e)?;
        let gas = DeviceBuffer::<u8>::zeroed(&stream, sizes.outputSizeInBytes).map_err(e)?;
        let mut handle: ox::OptixTraversableHandle = 0;
        check(
            "accelBuild",
            t.optixAccelBuild.unwrap()(
                dctx, stream.cu_stream() as _, &accel, &input, 1,
                temp.cu_deviceptr() as _, sizes.tempSizeInBytes,
                gas.cu_deviceptr() as _, sizes.outputSizeInBytes,
                &mut handle, ptr::null(), 0,
            ),
            &[0],
        )?;
        stream.synchronize().map_err(e)?;
        println!("GAS: {} bytes, handle {handle:#x}", sizes.outputSizeInBytes);

        // --- module, programs, pipeline ---
        let mco = ox::OptixModuleCompileOptions::default();
        let pco = ox::OptixPipelineCompileOptions {
            traversableGraphFlags: ox::OptixTraversableGraphFlags_OPTIX_TRAVERSABLE_GRAPH_FLAG_ALLOW_SINGLE_GAS as u32,
            numPayloadValues: 1,
            numAttributeValues: 2,
            exceptionFlags: ox::OptixExceptionFlags_OPTIX_EXCEPTION_FLAG_NONE as u32,
            pipelineLaunchParamsVariableName: c"params".as_ptr(),
            pipelineLaunchParamsSizeInBytes: size_of::<Params>(),
            usesPrimitiveTypeFlags: ox::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_TRIANGLE as u32,
            ..Default::default()
        };
        let mut module: ox::OptixModule = ptr::null_mut();
        log_len = log.len();
        let r = t.optixModuleCreate.unwrap()(
            dctx, &mco, &pco, ptx.as_ptr().cast(), ptx.len(), log.as_mut_ptr().cast(), &mut log_len, &mut module,
        );
        check("optixModuleCreate", r, &log)?;

        let mut descs = [ox::OptixProgramGroupDesc::default(), ox::OptixProgramGroupDesc::default(), ox::OptixProgramGroupDesc::default()];
        descs[0].kind = ox::OptixProgramGroupKind_OPTIX_PROGRAM_GROUP_KIND_RAYGEN;
        descs[0].__bindgen_anon_1.raygen = ox::OptixProgramGroupSingleModule { module, entryFunctionName: c"__raygen__mask".as_ptr() };
        descs[1].kind = ox::OptixProgramGroupKind_OPTIX_PROGRAM_GROUP_KIND_MISS;
        descs[1].__bindgen_anon_1.miss = ox::OptixProgramGroupSingleModule { module, entryFunctionName: c"__miss__none".as_ptr() };
        descs[2].kind = ox::OptixProgramGroupKind_OPTIX_PROGRAM_GROUP_KIND_HITGROUP;
        descs[2].__bindgen_anon_1.hitgroup = ox::OptixProgramGroupHitgroup {
            moduleCH: module,
            entryFunctionNameCH: c"__closesthit__hit".as_ptr(),
            ..Default::default()
        };
        let pgo = ox::OptixProgramGroupOptions::default();
        let mut groups: [ox::OptixProgramGroup; 3] = [ptr::null_mut(); 3];
        log_len = log.len();
        let r = t.optixProgramGroupCreate.unwrap()(dctx, descs.as_ptr(), 3, &pgo, log.as_mut_ptr().cast(), &mut log_len, groups.as_mut_ptr());
        check("optixProgramGroupCreate", r, &log)?;

        let plo = ox::OptixPipelineLinkOptions { maxTraceDepth: 1, ..Default::default() };
        let mut pipeline: ox::OptixPipeline = ptr::null_mut();
        log_len = log.len();
        let r = t.optixPipelineCreate.unwrap()(dctx, &pco, &plo, groups.as_ptr(), 3, log.as_mut_ptr().cast(), &mut log_len, &mut pipeline);
        check("optixPipelineCreate", r, &log)?;

        // --- SBT: header-only records ---
        let mut recs = [Record([0; SBT_HEADER]), Record([0; SBT_HEADER]), Record([0; SBT_HEADER])];
        for (g, rec) in groups.iter().zip(recs.iter_mut()) {
            check("packHeader", t.optixSbtRecordPackHeader.unwrap()(*g, rec.0.as_mut_ptr().cast()), &[0])?;
        }
        let rec_dev: Vec<_> = recs.iter().map(|r| DeviceBuffer::<u8>::from_host(&stream, &r.0)).collect::<Result<_, _>>().map_err(e)?;
        let sbt = ox::OptixShaderBindingTable {
            raygenRecord: rec_dev[0].cu_deviceptr() as _,
            missRecordBase: rec_dev[1].cu_deviceptr() as _,
            missRecordStrideInBytes: SBT_HEADER as u32,
            missRecordCount: 1,
            hitgroupRecordBase: rec_dev[2].cu_deviceptr() as _,
            hitgroupRecordStrideInBytes: SBT_HEADER as u32,
            hitgroupRecordCount: 1,
            ..Default::default()
        };

        let params = Params { out: out.cu_deviceptr() as u64, handle, width: W, height: H };
        let params_bytes = std::slice::from_raw_parts((&raw const params).cast::<u8>(), size_of::<Params>());
        let params_dev = DeviceBuffer::<u8>::from_host(&stream, params_bytes).map_err(e)?;
        stream.synchronize().map_err(e)?;

        let r = t.optixLaunch.unwrap()(pipeline, stream.cu_stream() as _, params_dev.cu_deviceptr() as _, size_of::<Params>(), &sbt, W, H, 1);
        check("optixLaunch", r, &[0])?;
        stream.synchronize().map_err(e)?;

        let got = out.to_host_vec(&stream).map_err(e)?;
        let want: Vec<u32> = (0..H)
            .flat_map(|y| (0..W).map(move |x| u32::from((x as f32 + 0.5) / W as f32 + (y as f32 + 0.5) / (H as f32) < LEG)))
            .collect();
        for y in (0..H).rev() {
            println!("{}", (0..W).map(|x| match got[(y * W + x) as usize] { 1 => '#', 0 => '.', _ => '?' }).collect::<String>());
        }
        let bad: Vec<_> = got.iter().zip(&want).enumerate().filter(|(_, (g, w))| g != w).map(|(i, (g, w))| (i, *g, *w)).collect();
        if !bad.is_empty() {
            return Err(format!("{} wrong pixels (index, got, want): {bad:?}", bad.len()));
        }
        println!("S1b PASS: a Rust raygen traced a one-triangle GAS; {W}x{H} hit mask matches the analytic mask");

        t.optixPipelineDestroy.unwrap()(pipeline);
        for g in groups {
            t.optixProgramGroupDestroy.unwrap()(g);
        }
        t.optixModuleDestroy.unwrap()(module);
        t.optixDeviceContextDestroy.unwrap()(dctx);
    }
    Ok(())
}
