//! [`Pipeline`]: linked program groups, and launching them.
//!
//! **What.** Links program groups compiled under one [`PipelineOptions`]; the pipeline holds its
//! groups. [`Pipeline::launch`] runs a `width x height x depth` grid of raygen invocations on a
//! stream with launch parameters read from a device buffer and an SBT.
//! **Stack sizes** are OptiX's defaults unless [`Pipeline::set_stack_size`] is called; deep
//! callable graphs need explicit sizes.

use crate::{DeviceContext, Error, LOG_BYTES, PipelineOptions, ProgramGroup, Result, ShaderBindingTable, check, check_log, entry, module::log_text, sys};
use cuda_core::{DeviceBuffer, Stream};
use std::sync::Arc;

/// Link-time limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkOptions {
    /// Maximum `optixTrace` recursion depth (1 = raygen traces, hit programs do not).
    pub max_trace_depth: u32,
}

impl Default for LinkOptions {
    fn default() -> Self {
        Self { max_trace_depth: 1 }
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
        let plo = sys::OptixPipelineLinkOptions { maxTraceDepth: link.max_trace_depth, ..Default::default() };
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
    /// GASes, 1 for a single GAS).
    pub fn set_stack_size(&self, direct_from_traversal: u32, direct_from_state: u32, continuation: u32, max_graph_depth: u32) -> Result<()> {
        // SAFETY: owned pipeline handle.
        check("optixPipelineSetStackSize", unsafe {
            entry!(self.ctx.table(), optixPipelineSetStackSize)(self.raw, direct_from_traversal, direct_from_state, continuation, max_graph_depth)
        })
    }

    /// Launches `dims` raygen invocations on `stream`.
    ///
    /// # Safety
    ///
    /// OptiX cannot check types across the host/device boundary: `params` must hold what the
    /// device code's launch-parameter struct expects (the size is checked), every SBT record's
    /// data must match what its programs read, and every device pointer and traversable handle
    /// reachable from them must stay valid until the launch completes on `stream`. `params` and
    /// `sbt` themselves must also outlive the launch (synchronize `stream` before dropping them,
    /// or drop them after later work on it).
    pub unsafe fn launch(&self, stream: &Stream, params: &DeviceBuffer<u8>, sbt: &ShaderBindingTable, dims: [u32; 3]) -> Result<()> {
        if params.len() != self.params_size {
            return Err(Error::Invalid("launch-parameter buffer size differs from PipelineOptions::params_size"));
        }
        if dims.iter().any(|d| *d == 0) || dims.iter().map(|d| u64::from(*d)).product::<u64>() > 1 << 30 {
            return Err(Error::Invalid("launch dimensions must be non-zero with at most 2^30 invocations"));
        }
        self.ctx.cuda().bind_to_thread()?;
        // SAFETY: the caller upholds the layout and lifetime contract above; the handles are owned.
        check("optixLaunch", unsafe {
            entry!(self.ctx.table(), optixLaunch)(self.raw, stream.cu_stream(), params.cu_deviceptr(), params.len(), sbt.raw(), dims[0], dims[1], dims[2])
        })
    }

    /// The raw handle.
    pub fn raw(&self) -> sys::OptixPipeline {
        self.raw
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
