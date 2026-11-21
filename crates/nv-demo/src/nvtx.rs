//! Optional header-only NVTX v3 range markers for Nsight Systems profiling.
//!
//! **What.** Provides a zero-allocation RAII thread-local range guard ([`NvtxRange`]), OS thread
//! naming ([`name_current_thread`]), and ARGB stage color constants (`COLOR_*`) for `OptiX RT`,
//! `Denoise`, `Bloom`, `Tonemap`, and `DLSS`.
//! **Why.** Lets Nsight Systems (`nsys profile`) display color-coded CPU/GPU render stages
//! without adding Cargo dependencies or requiring any runtime NVTX DLLs.
//! **How.** When `$CUDA_PATH/include/nvtx3/nvToolsExt.h` is found at build time, `build.rs`
//! compiles `nvtx_shim.c` and enables `cfg(has_nvtx3)`; at runtime NVTX v3 dynamically binds to
//! the profiler injection library via `NVTX_INJECTION64_PATH` or collapses to a single null check.
//! Without CUDA headers, all functions compile to inline no-ops.
//! **Used by** `renderer.rs` and `main.rs`.

use std::ffi::CStr;
use std::marker::PhantomData;

/// ARGB color for the OptiX ray-tracing pass (NVIDIA green).
pub const COLOR_RT: u32 = 0xFF76_B900;
/// ARGB color for the OptiX AI denoiser pass (cyan).
pub const COLOR_DENOISE: u32 = 0xFF00_B4D8;
/// ARGB color for the cuFFT bloom pass (amber).
pub const COLOR_BLOOM: u32 = 0xFFFF_B703;
/// ARGB color for the CUDA tonemap pass (violet).
pub const COLOR_TONEMAP: u32 = 0xFF9B_5DE5;
/// ARGB color for the NGX DLSS / DLSS-RR pass (magenta).
pub const COLOR_DLSS: u32 = 0xFFF1_5BB5;

#[cfg(has_nvtx3)]
unsafe extern "C" {
    fn nv_demo_nvtx_push(name: *const std::ffi::c_char, argb: u32);
    fn nv_demo_nvtx_pop();
    fn nv_demo_nvtx_name_thread(name: *const std::ffi::c_char);
}

/// RAII guard for a nested NVTX range on the current thread.
///
/// Intentionally `!Send` (`PhantomData<*mut ()>`) because NVTX push/pop ranges are tracked on a
/// per-thread stack and must be popped on the same thread that pushed them.
#[must_use = "bind the guard (`let _range = ...`) so the NVTX range stays open for the scope"]
pub struct NvtxRange {
    _not_send: PhantomData<*mut ()>,
}

impl NvtxRange {
    /// Pushes a color-coded NVTX range onto the current thread's stack until dropped.
    #[inline]
    pub fn new(name: &'static CStr, argb: u32) -> Self {
        #[cfg(has_nvtx3)]
        // SAFETY: `name` is a valid NUL-terminated `'static` C string; `nv_demo_nvtx_push`
        // only reads it during the call and never retains the pointer.
        unsafe {
            nv_demo_nvtx_push(name.as_ptr(), argb);
        }
        #[cfg(not(has_nvtx3))]
        let _ = (name, argb);
        Self {
            _not_send: PhantomData,
        }
    }
}

impl Drop for NvtxRange {
    #[inline]
    fn drop(&mut self) {
        #[cfg(has_nvtx3)]
        // SAFETY: Matches the push in `NvtxRange::new` on the same thread (`!Send`).
        unsafe {
            nv_demo_nvtx_pop();
        }
    }
}

/// Assigns a human-readable label to the current OS thread in Nsight Systems timelines.
#[inline]
pub fn name_current_thread(name: &'static CStr) {
    #[cfg(has_nvtx3)]
    // SAFETY: `name` is a valid NUL-terminated `'static` C string read synchronously by NVTX.
    unsafe {
        nv_demo_nvtx_name_thread(name.as_ptr());
    }
    #[cfg(not(has_nvtx3))]
    let _ = name;
}
