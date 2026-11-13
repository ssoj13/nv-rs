//! Spike S3 gate: the three sys crates built by nv-bindgen load and answer.
//! NVML: init + device count + name. OptiX: function table filled. cuFFT: version, and a
//! compile-time proof that cufft's cudaStream_t is cuda-bindings' CUstream.

fn main() -> Result<(), String> {
    // NVML (driver exports)
    let nvml = nvml_sys::load().map_err(|e| e.to_string())?;
    unsafe {
        let r = nvml.nvmlInit_v2();
        if r != nvml_sys::nvmlReturn_enum_NVML_SUCCESS { return Err(format!("nvmlInit_v2: {r}")); }
        let mut n = 0u32;
        nvml.nvmlDeviceGetCount_v2(&mut n);
        let mut dev: nvml_sys::nvmlDevice_t = std::ptr::null_mut();
        nvml.nvmlDeviceGetHandleByIndex_v2(0, &mut dev);
        let mut name = [0 as std::ffi::c_char; 96];
        nvml.nvmlDeviceGetName(dev, name.as_mut_ptr(), name.len() as u32);
        let name = std::ffi::CStr::from_ptr(name.as_ptr()).to_string_lossy();
        println!("NVML: {n} device(s); #0 = {name}");
        nvml.nvmlShutdown();
    }

    // OptiX (driver-store function table)
    let optix = optix_sys::load().map_err(|e| e.to_string())?;
    println!(
        "OptiX: ABI {} table loaded ({} bytes), optixDeviceContextCreate present: {}",
        optix_sys::OPTIX_ABI_VERSION,
        size_of::<optix_sys::OptixFunctionTable>(),
        optix.table.optixDeviceContextCreate.is_some()
    );

    // cuFFT (toolkit exports, shared CUDA stream type)
    let (cufft, path) = cufft_sys::load().map_err(|e| e.to_string())?;
    let mut v = 0i32;
    unsafe { cufft.cufftGetVersion(&mut v) };
    println!("cuFFT: version {v} from {}", path.display());
    let _same_type: cufft_sys::cudaStream_t = std::ptr::null_mut::<cuda_bindings::CUstream_st>();

    println!("S3 PASS: nvml (exports), optix (function table), cufft (toolkit + shared CUDA types) via one generator and one loader");
    Ok(())
}
