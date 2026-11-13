//! [`DeviceContext`]: OptiX bound to one CUDA context, its properties, log sink and disk cache.
//!
//! **What.** Creates the `OptixDeviceContext` on a `cuda_core::Context`, optionally with full
//! validation (slow; for debugging) and a log sink that receives OptiX's messages. Every other
//! object in this crate holds an `Arc<DeviceContext>`, so the context outlives them. The context
//! also answers device limits and capabilities (`optixDeviceContextGetProperty`, one typed
//! getter per `OptixDeviceProperty`), configures the module disk cache, cancels task-based module
//! creation ([`DeviceContext::cancel_creations`]) and names `OptixResult` codes
//! ([`error_name`], [`error_string`], used by [`crate::Error`]'s display).
//! **Why boxed sinks that are never freed early.** OptiX calls the log callback with a user
//! pointer; each sink lives in a `Box` owned by the context, so that pointer stays valid. OptiX
//! does not say when concurrent calls stop using a replaced callback, so
//! [`DeviceContext::set_log`] keeps every earlier sink alive until the context is dropped.
//! **Used by** every handle in this crate.

use crate::{Error, Result, check, entry, sys, table};
use cuda_core::Context;
use std::ffi::{CStr, CString, c_char, c_int, c_uint, c_void};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Receives OptiX log messages: `(level, tag, message)`; level 1 = fatal ... 4 = print.
pub type LogFn = dyn Fn(u32, &str, &str) + Send + Sync;

/// Highest OptiX log level (print).
const MAX_LOG_LEVEL: u32 = 4;

/// Bytes for [`DeviceContext::cache_location`]: Windows' extended-path limit of 32767 UTF-16
/// units at up to three UTF-8 bytes each, plus the NUL; also covers Linux `PATH_MAX`.
const PATH_BYTES: usize = 32767 * 3 + 1;

/// OptiX on one CUDA context.
pub struct DeviceContext {
    raw: sys::OptixDeviceContext,
    cuda: Arc<Context>,
    t: &'static sys::OptixFunctionTable,
    // Every sink ever installed; OptiX reads them through the pointers it holds, and the outer
    // box keeps each inner one at a fixed address while the vector grows.
    #[expect(clippy::vec_box, reason = "OptiX holds the address of each `Box<LogFn>`; the outer box keeps it fixed while the Vec reallocates")]
    sinks: Mutex<Vec<Box<Box<LogFn>>>>,
}

// SAFETY: OptiX documents device-context functions as thread safe; the sinks are Send + Sync and
// guarded by a mutex.
unsafe impl Send for DeviceContext {}
// SAFETY: as above.
unsafe impl Sync for DeviceContext {}

unsafe extern "C" fn forward_log(level: c_uint, tag: *const c_char, msg: *const c_char, data: *mut c_void) {
    if data.is_null() || tag.is_null() || msg.is_null() {
        return;
    }
    // SAFETY: `data` is a `Box<LogFn>` owned by the context for its whole life, alive while OptiX
    // may call back; `tag`/`msg` are non-null NUL-terminated strings valid for the call.
    let (sink, tag, msg) = unsafe { (&*data.cast::<Box<LogFn>>(), CStr::from_ptr(tag), CStr::from_ptr(msg)) };
    // A panicking sink must not unwind into OptiX; the panic was already reported by the hook.
    let _unwound = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| sink(level, &tag.to_string_lossy(), &msg.to_string_lossy())));
}

/// Callback, user pointer and boxed sink for `log`.
fn sink_parts(log: Option<Box<LogFn>>) -> (sys::OptixLogCallback, *mut c_void, Option<Box<Box<LogFn>>>) {
    let mut sink = log.map(Box::new);
    let data = sink.as_mut().map_or(std::ptr::null_mut(), |s| (&raw mut **s).cast::<c_void>());
    let callback: sys::OptixLogCallback = if sink.is_some() { Some(forward_log) } else { None };
    (callback, data, sink)
}

impl DeviceContext {
    /// OptiX on `cuda`. `log` receives messages up to `log_level` (0 = none, 4 = everything);
    /// `validation` turns on OptiX's full validation mode.
    pub fn new(cuda: &Arc<Context>, log: Option<Box<LogFn>>, log_level: u32, validation: bool) -> Result<Arc<Self>> {
        let t = table()?;
        cuda.bind_to_thread()?;
        let (callback, data, sink) = sink_parts(log);
        let opts = sys::OptixDeviceContextOptions {
            logCallbackFunction: callback,
            logCallbackData: data,
            logCallbackLevel: log_level.min(MAX_LOG_LEVEL) as i32,
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
        Ok(Arc::new(Self { raw, cuda: cuda.clone(), t, sinks: Mutex::new(sink.into_iter().collect()) }))
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

    /// Replaces the log sink (`None` silences OptiX) and its level (0 = none ... 4 = print).
    ///
    /// OptiX swaps callback and data atomically but not together with the level, and concurrent
    /// calls on other threads may keep using the previous sink for a while; earlier sinks
    /// therefore stay alive (and may still be called) until the context is dropped.
    pub fn set_log(&self, log: Option<Box<LogFn>>, level: u32) -> Result<()> {
        let (callback, data, sink) = sink_parts(log);
        let mut sinks = self.sinks.lock().map_err(|_| Error::Invalid("log sink list poisoned by a panic"))?;
        // SAFETY: owned context; the new sink is pushed below and kept for the context's life, the
        // old ones were never released.
        check("optixDeviceContextSetLogCallback", unsafe {
            entry!(self.t, optixDeviceContextSetLogCallback)(self.raw, callback, data, level.min(MAX_LOG_LEVEL))
        })?;
        sinks.extend(sink);
        Ok(())
    }

    fn prop(&self, property: sys::OptixDeviceProperty) -> Result<u32> {
        let mut value: c_uint = 0;
        // SAFETY: every `OptixDeviceProperty` is documented as `sizeof(unsigned int)`; `value` is a
        // writable `unsigned int`.
        check("optixDeviceContextGetProperty", unsafe {
            entry!(self.t, optixDeviceContextGetProperty)(self.raw, property, (&raw mut value).cast(), size_of::<c_uint>())
        })?;
        Ok(value)
    }

    fn flag(&self, property: sys::OptixDeviceProperty, standard: c_int) -> Result<bool> {
        Ok(self.prop(property)? & standard as u32 != 0)
    }

    /// Largest `LinkOptions::max_trace_depth` (`OPTIX_DEVICE_PROPERTY_LIMIT_MAX_TRACE_DEPTH`).
    pub fn max_trace_depth(&self) -> Result<u32> {
        self.prop(sys::OptixDeviceProperty_OPTIX_DEVICE_PROPERTY_LIMIT_MAX_TRACE_DEPTH)
    }

    /// Largest traversable graph depth for stack-size setup
    /// (`OPTIX_DEVICE_PROPERTY_LIMIT_MAX_TRAVERSABLE_GRAPH_DEPTH`).
    pub fn max_graph_depth(&self) -> Result<u32> {
        self.prop(sys::OptixDeviceProperty_OPTIX_DEVICE_PROPERTY_LIMIT_MAX_TRAVERSABLE_GRAPH_DEPTH)
    }

    /// Most primitives over all build inputs of one GAS
    /// (`OPTIX_DEVICE_PROPERTY_LIMIT_MAX_PRIMITIVES_PER_GAS`).
    pub fn max_prims_per_gas(&self) -> Result<u32> {
        self.prop(sys::OptixDeviceProperty_OPTIX_DEVICE_PROPERTY_LIMIT_MAX_PRIMITIVES_PER_GAS)
    }

    /// Most instances over all build inputs of one IAS
    /// (`OPTIX_DEVICE_PROPERTY_LIMIT_MAX_INSTANCES_PER_IAS`).
    pub fn max_instances_per_ias(&self) -> Result<u32> {
        self.prop(sys::OptixDeviceProperty_OPTIX_DEVICE_PROPERTY_LIMIT_MAX_INSTANCES_PER_IAS)
    }

    /// RT core version: 0 = none, 10 = 1.0, ... (`OPTIX_DEVICE_PROPERTY_RTCORE_VERSION`).
    pub fn rtcore_version(&self) -> Result<u32> {
        self.prop(sys::OptixDeviceProperty_OPTIX_DEVICE_PROPERTY_RTCORE_VERSION)
    }

    /// Largest `OptixInstance::instanceId` (`OPTIX_DEVICE_PROPERTY_LIMIT_MAX_INSTANCE_ID`).
    pub fn max_instance_id(&self) -> Result<u32> {
        self.prop(sys::OptixDeviceProperty_OPTIX_DEVICE_PROPERTY_LIMIT_MAX_INSTANCE_ID)
    }

    /// Usable bits of `OptixInstance::visibilityMask`
    /// (`OPTIX_DEVICE_PROPERTY_LIMIT_NUM_BITS_INSTANCE_VISIBILITY_MASK`).
    pub fn visibility_mask_bits(&self) -> Result<u32> {
        self.prop(sys::OptixDeviceProperty_OPTIX_DEVICE_PROPERTY_LIMIT_NUM_BITS_INSTANCE_VISIBILITY_MASK)
    }

    /// Most SBT records of one GAS (`OPTIX_DEVICE_PROPERTY_LIMIT_MAX_SBT_RECORDS_PER_GAS`).
    pub fn max_sbt_records_per_gas(&self) -> Result<u32> {
        self.prop(sys::OptixDeviceProperty_OPTIX_DEVICE_PROPERTY_LIMIT_MAX_SBT_RECORDS_PER_GAS)
    }

    /// Largest summed SBT offset along a traversable path
    /// (`OPTIX_DEVICE_PROPERTY_LIMIT_MAX_SBT_OFFSET`).
    pub fn max_sbt_offset(&self) -> Result<u32> {
        self.prop(sys::OptixDeviceProperty_OPTIX_DEVICE_PROPERTY_LIMIT_MAX_SBT_OFFSET)
    }

    /// `optixReorder` really reorders threads (false: it is a legal no-op)
    /// (`OPTIX_DEVICE_PROPERTY_SHADER_EXECUTION_REORDERING`, flag `STANDARD`).
    pub fn shader_reordering(&self) -> Result<bool> {
        self.flag(
            sys::OptixDeviceProperty_OPTIX_DEVICE_PROPERTY_SHADER_EXECUTION_REORDERING,
            sys::OptixDevicePropertyShaderExecutionReorderingFlags_OPTIX_DEVICE_PROPERTY_SHADER_EXECUTION_REORDERING_FLAG_STANDARD,
        )
    }

    /// Cooperative vectors are supported (`OPTIX_DEVICE_PROPERTY_COOP_VEC`, flag `STANDARD`).
    pub fn coop_vec(&self) -> Result<bool> {
        self.flag(sys::OptixDeviceProperty_OPTIX_DEVICE_PROPERTY_COOP_VEC, sys::OptixDevicePropertyCoopVecFlags_OPTIX_DEVICE_PROPERTY_COOP_VEC_FLAG_STANDARD)
    }

    /// Cluster acceleration structure builds are supported (`OPTIX_DEVICE_PROPERTY_CLUSTER_ACCEL`,
    /// flag `STANDARD`).
    pub fn cluster_accel(&self) -> Result<bool> {
        self.flag(
            sys::OptixDeviceProperty_OPTIX_DEVICE_PROPERTY_CLUSTER_ACCEL,
            sys::OptixDevicePropertyClusterAccelFlags_OPTIX_DEVICE_PROPERTY_CLUSTER_ACCEL_FLAG_STANDARD,
        )
    }

    /// Most unique vertices per cluster (`OPTIX_DEVICE_PROPERTY_LIMIT_MAX_CLUSTER_VERTICES`).
    pub fn max_cluster_vertices(&self) -> Result<u32> {
        self.prop(sys::OptixDeviceProperty_OPTIX_DEVICE_PROPERTY_LIMIT_MAX_CLUSTER_VERTICES)
    }

    /// Most triangles per cluster (`OPTIX_DEVICE_PROPERTY_LIMIT_MAX_CLUSTER_TRIANGLES`).
    pub fn max_cluster_triangles(&self) -> Result<u32> {
        self.prop(sys::OptixDeviceProperty_OPTIX_DEVICE_PROPERTY_LIMIT_MAX_CLUSTER_TRIANGLES)
    }

    /// Largest grid resolution of a structured cluster
    /// (`OPTIX_DEVICE_PROPERTY_LIMIT_MAX_STRUCTURED_GRID_RESOLUTION`).
    pub fn max_grid_resolution(&self) -> Result<u32> {
        self.prop(sys::OptixDeviceProperty_OPTIX_DEVICE_PROPERTY_LIMIT_MAX_STRUCTURED_GRID_RESOLUTION)
    }

    /// Largest SBT index in a cluster build (`OPTIX_DEVICE_PROPERTY_LIMIT_MAX_CLUSTER_SBT_INDEX`).
    pub fn max_cluster_sbt_index(&self) -> Result<u32> {
        self.prop(sys::OptixDeviceProperty_OPTIX_DEVICE_PROPERTY_LIMIT_MAX_CLUSTER_SBT_INDEX)
    }

    /// Most clusters as input to one GAS (`OPTIX_DEVICE_PROPERTY_LIMIT_MAX_CLUSTERS_PER_GAS`).
    pub fn max_clusters_per_gas(&self) -> Result<u32> {
        self.prop(sys::OptixDeviceProperty_OPTIX_DEVICE_PROPERTY_LIMIT_MAX_CLUSTERS_PER_GAS)
    }

    /// Turns the module disk cache on or off. Enabling (re)opens the database at the current
    /// location and fails if that is impossible. `OPTIX_CACHE_MAXSIZE=0` in the environment
    /// disables the cache regardless.
    pub fn set_cache_enabled(&self, enabled: bool) -> Result<()> {
        // SAFETY: owned context; plain value argument.
        check("optixDeviceContextSetCacheEnabled", unsafe { entry!(self.t, optixDeviceContextSetCacheEnabled)(self.raw, c_int::from(enabled)) })
    }

    /// Whether the disk cache is on.
    pub fn cache_enabled(&self) -> Result<bool> {
        let mut enabled: c_int = 0;
        // SAFETY: owned context; writable `int`.
        check("optixDeviceContextGetCacheEnabled", unsafe { entry!(self.t, optixDeviceContextGetCacheEnabled)(self.raw, &mut enabled) })?;
        Ok(enabled != 0)
    }

    /// Moves the disk cache to the directory `dir` (created if missing; must not be a network
    /// share). `OPTIX_CACHE_PATH` in the environment takes precedence.
    pub fn set_cache_location(&self, dir: &Path) -> Result<()> {
        let dir = dir.to_str().ok_or(Error::Invalid("cache location is not valid UTF-8"))?;
        let dir = CString::new(dir).map_err(|_| Error::Invalid("cache location contains NUL"))?;
        // SAFETY: owned context; NUL-terminated string alive for the call (OptiX copies it).
        check("optixDeviceContextSetCacheLocation", unsafe { entry!(self.t, optixDeviceContextSetCacheLocation)(self.raw, dir.as_ptr()) })
    }

    /// The disk cache directory (empty when `OPTIX_CACHE_MAXSIZE=0` disabled the cache).
    pub fn cache_location(&self) -> Result<PathBuf> {
        let mut buf = vec![0u8; PATH_BYTES];
        // SAFETY: owned context; `buf` is writable for the size passed, and OptiX NUL-terminates.
        check("optixDeviceContextGetCacheLocation", unsafe {
            entry!(self.t, optixDeviceContextGetCacheLocation)(self.raw, buf.as_mut_ptr().cast(), buf.len())
        })?;
        let text = CStr::from_bytes_until_nul(&buf).map_err(|_| Error::Invalid("cache location not NUL-terminated"))?;
        let text = text.to_str().map_err(|_| Error::Invalid("cache location is not valid UTF-8"))?;
        Ok(PathBuf::from(text))
    }

    /// Garbage-collection water marks in bytes: when an insert would exceed `high`, entries are
    /// evicted down to `low`; 0 for either disables collection; `high < low` (both non-zero) is
    /// an error. `OPTIX_CACHE_MAXSIZE` overrides them.
    pub fn set_cache_sizes(&self, low: usize, high: usize) -> Result<()> {
        // SAFETY: owned context; plain value arguments.
        check("optixDeviceContextSetCacheDatabaseSizes", unsafe { entry!(self.t, optixDeviceContextSetCacheDatabaseSizes)(self.raw, low, high) })
    }

    /// The `(low, high)` water marks (both 0 when the environment disabled the cache).
    pub fn cache_sizes(&self) -> Result<(usize, usize)> {
        let (mut low, mut high) = (0usize, 0usize);
        // SAFETY: owned context; two writable `size_t`s.
        check("optixDeviceContextGetCacheDatabaseSizes", unsafe {
            entry!(self.t, optixDeviceContextGetCacheDatabaseSizes)(self.raw, &mut low, &mut high)
        })?;
        Ok((low, high))
    }

    /// Cancels every task-based module creation on this context (see
    /// [`crate::PendingModule::cancel`]); `block` waits until the cancellation is effective.
    /// Safe from any thread.
    pub fn cancel_creations(&self, block: bool) -> Result<()> {
        // SAFETY: owned context; OptiX documents this call as thread safe.
        check("optixDeviceContextCancelCreations", unsafe { entry!(self.t, optixDeviceContextCancelCreations)(self.raw, creation_flags(block)) })
    }
}

/// `OptixCreationFlags` for a cancel call.
pub(crate) fn creation_flags(block: bool) -> sys::OptixCreationFlags {
    if block { sys::OptixCreationFlags_OPTIX_CREATION_FLAG_BLOCK_UNTIL_EFFECTIVE } else { sys::OptixCreationFlags_OPTIX_CREATION_FLAG_NONE }
}

impl Drop for DeviceContext {
    fn drop(&mut self) {
        if let Some(destroy) = self.t.optixDeviceContextDestroy {
            // SAFETY: owned handle; every object created from it holds an `Arc` of `self`, so
            // none is alive any more. The sinks are dropped after this, when OptiX is done.
            let code = unsafe { destroy(self.raw) };
            self.record(code);
        }
    }
}

/// Reads a static string returned by `optixGetErrorName` / `optixGetErrorString`.
fn error_text(f: Option<unsafe extern "C" fn(sys::OptixResult) -> *const c_char>, name: &'static str, code: sys::OptixResult) -> Result<&'static str> {
    let f = f.ok_or_else(|| Error::Unavailable(format!("{name} missing from the OptiX function table")))?;
    // SAFETY: the entry takes any code and returns a pointer to a static string (or null).
    let p = unsafe { f(code) };
    if p.is_null() {
        return Err(Error::Invalid("OptiX returned no error text"));
    }
    // SAFETY: non-null, NUL-terminated, static in the driver library, which `optix-sys` keeps
    // loaded for the life of the process.
    let text = unsafe { CStr::from_ptr(p) };
    text.to_str().map_err(|_| Error::Invalid("OptiX error text is not UTF-8"))
}

/// The enum name of an `OptixResult` (`"OPTIX_ERROR_INVALID_VALUE"`; OptiX answers
/// `"Unknown OptixResult code"` for unknown codes).
pub fn error_name(code: sys::OptixResult) -> Result<&'static str> {
    error_text(table()?.optixGetErrorName, "optixGetErrorName", code)
}

/// The description of an `OptixResult` (`"Invalid value"`).
pub fn error_string(code: sys::OptixResult) -> Result<&'static str> {
    error_text(table()?.optixGetErrorString, "optixGetErrorString", code)
}

/// `"<name> (<code>): <description>"`, as [`crate::Error`]'s display shows it; when the driver
/// cannot name the code, says so next to the raw number instead of hiding it.
pub fn describe(code: &sys::OptixResult) -> String {
    match (error_name(*code), error_string(*code)) {
        (Ok(name), Ok(text)) => format!("{name} ({code}): {text}"),
        (Err(e), _) | (_, Err(e)) => format!("OptixResult {code} (no name: {e})"),
    }
}
