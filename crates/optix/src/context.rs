//! [`DeviceContext`]: OptiX bound to one CUDA context.
//!
//! **What.** Creates the `OptixDeviceContext` on a `cuda_core::Context`, optionally with full
//! validation (slow; for debugging) and a log sink that receives OptiX's messages. Every other
//! object in this crate holds an `Arc<DeviceContext>`, so the context outlives them.
//! **Why a boxed sink.** OptiX calls the log callback with a user pointer for the life of the
//! context; the sink lives in a `Box` owned by the context, so that pointer stays valid.

use crate::{Result, check, entry, sys, table};
use cuda_core::Context;
use std::ffi::{CStr, c_char, c_uint, c_void};
use std::sync::Arc;

/// Receives OptiX log messages: `(level, tag, message)`; level 1 = fatal ... 4 = print.
pub type LogFn = dyn Fn(u32, &str, &str) + Send + Sync;

/// OptiX on one CUDA context.
pub struct DeviceContext {
    raw: sys::OptixDeviceContext,
    cuda: Arc<Context>,
    t: &'static sys::OptixFunctionTable,
    // Read through the pointer OptiX holds; never moved while `raw` lives.
    _sink: Option<Box<Box<LogFn>>>,
}

// SAFETY: OptiX documents device-context functions as thread safe; the sink is Send + Sync.
unsafe impl Send for DeviceContext {}
// SAFETY: as above.
unsafe impl Sync for DeviceContext {}

unsafe extern "C" fn forward_log(level: c_uint, tag: *const c_char, msg: *const c_char, data: *mut c_void) {
    if data.is_null() {
        return;
    }
    // SAFETY: `data` is the `Box<LogFn>` owned by the context, alive while OptiX may call back;
    // `tag`/`msg` are NUL-terminated strings valid for the call.
    let (sink, tag, msg) = unsafe { (&*data.cast::<Box<LogFn>>(), CStr::from_ptr(tag), CStr::from_ptr(msg)) };
    // A panicking sink must not unwind into OptiX.
    let _unwound = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| sink(level, &tag.to_string_lossy(), &msg.to_string_lossy())));
}

impl DeviceContext {
    /// OptiX on `cuda`. `log` receives messages up to `log_level` (0 = none, 4 = everything);
    /// `validation` turns on OptiX's full validation mode.
    pub fn new(cuda: &Arc<Context>, log: Option<Box<LogFn>>, log_level: u32, validation: bool) -> Result<Arc<Self>> {
        let t = table()?;
        cuda.bind_to_thread()?;
        let mut sink = log.map(Box::new);
        let data = sink.as_mut().map_or(std::ptr::null_mut(), |s| (&raw mut **s).cast::<c_void>());
        let opts = sys::OptixDeviceContextOptions {
            logCallbackFunction: if sink.is_some() { Some(forward_log) } else { None },
            logCallbackData: data,
            logCallbackLevel: log_level.min(4) as i32,
            validationMode: if validation {
                sys::OptixDeviceContextValidationMode_OPTIX_DEVICE_CONTEXT_VALIDATION_MODE_ALL
            } else {
                sys::OptixDeviceContextValidationMode_OPTIX_DEVICE_CONTEXT_VALIDATION_MODE_OFF
            },
        };
        let mut raw = std::ptr::null_mut();
        // SAFETY: `cuda` is current and alive (held below); `opts` and the sink outlive the call,
        // and the sink lives as long as the context.
        check("optixDeviceContextCreate", unsafe { entry!(t, optixDeviceContextCreate)(cuda.cu_ctx(), &opts, &mut raw) })?;
        Ok(Arc::new(Self { raw, cuda: cuda.clone(), t, _sink: sink }))
    }

    /// The raw handle.
    pub fn raw(&self) -> sys::OptixDeviceContext {
        self.raw
    }

    /// The CUDA context OptiX runs on.
    pub fn cuda(&self) -> &Arc<Context> {
        &self.cuda
    }

    /// The driver's function table.
    pub fn table(&self) -> &'static sys::OptixFunctionTable {
        self.t
    }

    /// Records a failed destroy call on the CUDA context (drop paths cannot return errors).
    pub(crate) fn record(&self, code: sys::OptixResult) {
        if code != sys::OptixResult_OPTIX_SUCCESS {
            self.cuda.record_err(Err::<(), _>(cuda_core::DriverError(cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN)));
        }
    }
}

impl Drop for DeviceContext {
    fn drop(&mut self) {
        if let Some(destroy) = self.t.optixDeviceContextDestroy {
            // SAFETY: owned handle; every object created from it holds an `Arc` of `self`, so
            // none is alive any more.
            let code = unsafe { destroy(self.raw) };
            self.record(code);
        }
    }
}
