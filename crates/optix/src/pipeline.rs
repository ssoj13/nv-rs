//! [`Pipeline`]: linked program groups, launching them, and their global symbols.
//!
//! **What.** Links program groups compiled under one [`PipelineOptions`] with the call-depth
//! limits of [`LinkOptions`]; the pipeline holds its groups. [`Pipeline::launch`] runs a
//! `width x height x depth` grid of raygen invocations on a stream with launch parameters read
//! from a device buffer and an SBT. [`Pipeline::write_symbol`] / [`Pipeline::read_symbol`] (and
//! the device-buffer variants) copy to and from device globals of the linked modules
//! (`optixPipelineSymbolMemcpyAsync`).
//! **Stack sizes** are computed by OptiX from the [`LinkOptions`] depths at link time; change
//! them with [`Pipeline::set_stack_from_depths`], or compute exact sizes with `stack.rs`
//! ([`crate::StackSizes`]) and apply them with [`Pipeline::set_stack_size`].

use crate::{DeviceContext, Error, LOG_BYTES, PipelineOptions, ProgramGroup, Result, ShaderBindingTable, check, check_log, entry, module::log_text, sys};
use cuda_core::bytemuck::Pod;
use cuda_core::{DeviceBuffer, Stream};
use std::ffi::{CString, c_void};
use std::sync::Arc;

/// Link-time call-depth limits (`OptixPipelineLinkOptions`); OptiX sizes the stacks from them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkOptions {
    /// Maximum `optixTrace` recursion depth (0 = raygen cannot trace; 1 = raygen traces, hit
    /// programs do not; at most [`DeviceContext::max_trace_depth`]).
    pub max_trace_depth: u32,
    /// Maximum depth of continuation-callable call graphs (0 = none can be called).
    pub max_cc_depth: u32,
    /// Maximum depth of direct-callable call graphs called from raygen, closest hit, miss or
    /// continuation callables.
    pub max_dc_depth_from_state: u32,
    /// Maximum depth of direct-callable call graphs called from intersection or any hit.
    pub max_dc_depth_from_traversal: u32,
    /// Maximum traversable graph depth (IAS -> GAS = 2, IAS -> motion transform -> GAS = 3);
    /// 0 = 1 for single-GAS pipelines, else 2.
    pub max_graph_depth: u32,
}

impl Default for LinkOptions {
    /// Raygen traces, no callables, graph depth from the pipeline's graph flags.
    fn default() -> Self {
        Self { max_trace_depth: 1, max_cc_depth: 0, max_dc_depth_from_state: 0, max_dc_depth_from_traversal: 0, max_graph_depth: 0 }
    }
}

impl LinkOptions {
    fn to_sys(self) -> sys::OptixPipelineLinkOptions {
        sys::OptixPipelineLinkOptions {
            maxTraceDepth: self.max_trace_depth,
            maxContinuationCallableDepth: self.max_cc_depth,
            maxDirectCallableDepthFromState: self.max_dc_depth_from_state,
            maxDirectCallableDepthFromTraversal: self.max_dc_depth_from_traversal,
            maxTraversableGraphDepth: self.max_graph_depth,
        }
    }
}

/// Linked programs, ready to launch.
pub struct Pipeline {
    raw: sys::OptixPipeline,
    params_size: usize,
    ctx: Arc<DeviceContext>,
    _groups: Vec<Arc<ProgramGroup>>,
}

// SAFETY: an OptixPipeline is an immutable handle once linked; OptiX allows concurrent launches.
unsafe impl Send for Pipeline {}
// SAFETY: as above.
unsafe impl Sync for Pipeline {}

impl Pipeline {
    /// Links `groups`. Returns the pipeline and the linker log.
    pub fn new(ctx: &Arc<DeviceContext>, options: &PipelineOptions, link: LinkOptions, groups: &[Arc<ProgramGroup>]) -> Result<(Self, String)> {
        if groups.iter().any(|g| !Arc::ptr_eq(g.context(), ctx)) {
            return Err(Error::Invalid("pipeline mixes device contexts"));
        }
        let t = ctx.table();
        let pco = options.to_sys()?;
        let plo = link.to_sys();
        let raw_groups: Vec<sys::OptixProgramGroup> = groups.iter().map(|g| g.raw()).collect();
        let n = u32::try_from(raw_groups.len()).map_err(|_| Error::Invalid("too many program groups"))?;
        let mut log = vec![0u8; LOG_BYTES];
        let mut log_len = log.len();
        let mut raw = std::ptr::null_mut();
        ctx.cuda().bind_to_thread()?;
        // SAFETY: options, group handles and log buffer outlive the call.
        let code = unsafe {
            entry!(t, optixPipelineCreate)(ctx.raw(), &pco, &plo, raw_groups.as_ptr(), n, log.as_mut_ptr().cast(), &mut log_len, &mut raw)
        };
        check_log("optixPipelineCreate", code, &log, log_len)?;
        let pipeline = Self { raw, params_size: options.params_size, ctx: ctx.clone(), _groups: groups.to_vec() };
        Ok((pipeline, log_text(&log, log_len)))
    }

    /// Sets the stack sizes (bytes) and the maximum traversable graph depth (2 for one IAS over
    /// GASes, 1 for a single GAS). See [`crate::StackSizes`] for computing them.
    pub fn set_stack_size(&self, direct_from_traversal: u32, direct_from_state: u32, continuation: u32, max_graph_depth: u32) -> Result<()> {
        // SAFETY: owned pipeline handle.
        check("optixPipelineSetStackSize", unsafe {
            entry!(self.ctx.table(), optixPipelineSetStackSize)(self.raw, direct_from_traversal, direct_from_state, continuation, max_graph_depth)
        })
    }

    /// Recomputes the stack sizes from new call depths, as at link time
    /// (`optixPipelineSetStackSizeFromCallDepths`).
    pub fn set_stack_from_depths(&self, depths: LinkOptions) -> Result<()> {
        // SAFETY: owned pipeline handle; plain value arguments.
        check("optixPipelineSetStackSizeFromCallDepths", unsafe {
            entry!(self.ctx.table(), optixPipelineSetStackSizeFromCallDepths)(
                self.raw,
                depths.max_trace_depth,
                depths.max_cc_depth,
                depths.max_dc_depth_from_state,
                depths.max_dc_depth_from_traversal,
                depths.max_graph_depth,
            )
        })
    }

    /// Launches `dims` raygen invocations on `stream`.
    ///
    /// # Safety
    ///
    /// OptiX cannot check types across the host/device boundary: `params` must hold what the
    /// device code's launch-parameter struct expects (the size is checked), every SBT record's
    /// data and every device global written with [`Pipeline::write_symbol`] must match what the
    /// programs read, and every device pointer and traversable handle reachable from them must
    /// stay valid until the launch completes on `stream`. `params` and `sbt` themselves must also
    /// outlive the launch (synchronize `stream` before dropping them, or drop them after later
    /// work on it).
    pub unsafe fn launch(&self, stream: &Stream, params: &DeviceBuffer<u8>, sbt: &ShaderBindingTable, dims: [u32; 3]) -> Result<()> {
        if params.len() != self.params_size {
            return Err(Error::Invalid("launch-parameter buffer size differs from PipelineOptions::params_size"));
        }
        if dims.contains(&0) || dims.iter().map(|d| u64::from(*d)).product::<u64>() > 1 << 30 {
            return Err(Error::Invalid("launch dimensions must be non-zero with at most 2^30 invocations"));
        }
        self.ctx.cuda().bind_to_thread()?;
        // SAFETY: the caller upholds the layout and lifetime contract above; the handles are owned.
        check("optixLaunch", unsafe {
            entry!(self.ctx.table(), optixLaunch)(self.raw, stream.cu_stream(), params.cu_deviceptr(), params.len(), sbt.raw(), dims[0], dims[1], dims[2])
        })
    }

    /// One `optixPipelineSymbolMemcpyAsync` of `size` bytes at `offset` in symbol `name`.
    ///
    /// # Safety
    ///
    /// `mem` is host or device memory as `kind` says, valid for `size` bytes (read or written as
    /// `kind` says) until the copy completes on `stream`.
    unsafe fn symbol_copy(&self, stream: &Stream, name: &str, offset: usize, mem: *mut c_void, size: usize, kind: sys::OptixPipelineSymbolMemcpyKind) -> Result<()> {
        if size == 0 {
            return Err(Error::Invalid("symbol copy of zero bytes"));
        }
        if stream.context().cu_ctx() != self.ctx.cuda().cu_ctx() {
            return Err(Error::Invalid("stream belongs to another CUDA context than the pipeline"));
        }
        let name = CString::new(name).map_err(|_| Error::Invalid("symbol name contains NUL"))?;
        self.ctx.cuda().bind_to_thread()?;
        // SAFETY: owned pipeline; NUL-terminated name alive for the call; `mem` per the caller.
        check("optixPipelineSymbolMemcpyAsync", unsafe {
            entry!(self.ctx.table(), optixPipelineSymbolMemcpyAsync)(self.raw, name.as_ptr(), mem, size, offset, kind, stream.cu_stream())
        })
    }

    /// Writes `value` into device global `name` at byte `offset`, ordered on `stream`; returns
    /// once the copy is done (it synchronizes `stream`). Launches on other streams that read the
    /// global race with the write.
    pub fn write_symbol<T: Pod>(&self, stream: &Stream, name: &str, offset: usize, value: &T) -> Result<()> {
        let staged = Box::new(*value);
        // SAFETY: `staged` is `size_of::<T>()` readable host bytes, kept alive until the stream
        // has synchronized (or leaked if that fails, as the copy may still be reading it).
        unsafe {
            self.symbol_copy(
                stream,
                name,
                offset,
                (&raw const *staged).cast_mut().cast(),
                size_of::<T>(),
                sys::OptixPipelineSymbolMemcpyKind_OPTIX_PIPELINE_SYMBOL_MEMCPY_KIND_FROM_HOST,
            )?
        };
        if let Err(e) = stream.synchronize() {
            std::mem::forget(staged);
            return Err(e.into());
        }
        Ok(())
    }

    /// Reads a `T` from device global `name` at byte `offset`, ordered on `stream` (synchronizes
    /// it).
    pub fn read_symbol<T: Pod>(&self, stream: &Stream, name: &str, offset: usize) -> Result<T> {
        let mut out = Box::new(T::zeroed());
        // SAFETY: `out` is `size_of::<T>()` writable host bytes, kept alive until the stream has
        // synchronized (or leaked if that fails, as the copy may still be writing it); any bytes
        // are a valid `T` (`Pod`).
        unsafe {
            self.symbol_copy(
                stream,
                name,
                offset,
                (&raw mut *out).cast(),
                size_of::<T>(),
                sys::OptixPipelineSymbolMemcpyKind_OPTIX_PIPELINE_SYMBOL_MEMCPY_KIND_TO_HOST,
            )?
        };
        if let Err(e) = stream.synchronize() {
            std::mem::forget(out);
            return Err(e.into());
        }
        Ok(*out)
    }

    /// Copies all of `src` into device global `name` at byte `offset`, asynchronously on
    /// `stream`.
    ///
    /// # Safety
    ///
    /// `src` must stay alive and unmodified until the copy completes on `stream`.
    pub unsafe fn write_symbol_from(&self, stream: &Stream, name: &str, offset: usize, src: &DeviceBuffer<u8>) -> Result<()> {
        // SAFETY: a device pointer to `src.len()` bytes, kept valid by the caller.
        unsafe {
            self.symbol_copy(
                stream,
                name,
                offset,
                src.cu_deviceptr() as *mut c_void,
                src.len(),
                sys::OptixPipelineSymbolMemcpyKind_OPTIX_PIPELINE_SYMBOL_MEMCPY_KIND_FROM_DEVICE,
            )
        }
    }

    /// Copies `dst.len()` bytes of device global `name` from byte `offset` into `dst`,
    /// asynchronously on `stream`.
    ///
    /// # Safety
    ///
    /// `dst` must stay alive and must not be accessed until the copy completes on `stream`.
    pub unsafe fn read_symbol_into(&self, stream: &Stream, name: &str, offset: usize, dst: &mut DeviceBuffer<u8>) -> Result<()> {
        // SAFETY: a device pointer to `dst.len()` writable bytes, kept valid by the caller.
        unsafe {
            self.symbol_copy(
                stream,
                name,
                offset,
                dst.cu_deviceptr() as *mut c_void,
                dst.len(),
                sys::OptixPipelineSymbolMemcpyKind_OPTIX_PIPELINE_SYMBOL_MEMCPY_KIND_TO_DEVICE,
            )
        }
    }

    /// The raw handle.
    pub fn raw(&self) -> sys::OptixPipeline {
        self.raw
    }

    /// The context this pipeline was linked on.
    pub fn context(&self) -> &Arc<DeviceContext> {
        &self.ctx
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        if let Some(destroy) = self.ctx.table().optixPipelineDestroy {
            // SAFETY: owned handle; launches must have completed (the `launch` contract).
            let code = unsafe { destroy(self.raw) };
            self.ctx.record(code);
        }
    }
}
