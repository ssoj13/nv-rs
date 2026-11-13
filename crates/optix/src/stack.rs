//! Stack sizing: per-group stack needs ([`ProgramGroup::stack_sizes`]) and a port of NVIDIA's
//! `optix_stack_size.h` helpers.
//!
//! **What.** [`StackSizes`] is `OptixStackSizes` (continuation stack per program kind, direct
//! stack of direct callables). [`StackSizes::accumulate`] is `optixUtilAccumulateStackSizes`;
//! [`StackSizes::compute`], [`StackSizes::compute_dc_split`] and
//! [`StackSizes::compute_css_cc_tree`] are the `optixUtilComputeStackSizes*` formulas;
//! [`simple_path_tracer`] is `optixUtilComputeStackSizesSimplePathTracer`. The result,
//! [`PipelineStack`], goes to [`Pipeline::set_stack`].
//! **Why a port.** The header is inline C++ (no driver entry point); the formulas are copied
//! exactly, except that `unsigned` wrap-around becomes a typed error.
//! **Used by** pipelines with recursion, callables or motion transforms, whose default stack
//! sizes are not sufficient.

use crate::{Error, Pipeline, ProgramGroup, Result, check, entry, sys};
use std::sync::Arc;

/// Stack needs of a program group, or the maximum over several (`OptixStackSizes`), in bytes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StackSizes {
    /// Continuation stack of raygen programs.
    pub css_rg: u32,
    /// Continuation stack of miss programs.
    pub css_ms: u32,
    /// Continuation stack of closest-hit programs.
    pub css_ch: u32,
    /// Continuation stack of any-hit programs.
    pub css_ah: u32,
    /// Continuation stack of intersection programs.
    pub css_is: u32,
    /// Continuation stack of continuation callables.
    pub css_cc: u32,
    /// Direct stack of direct callables.
    pub dss_dc: u32,
}

impl From<sys::OptixStackSizes> for StackSizes {
    fn from(s: sys::OptixStackSizes) -> Self {
        Self { css_rg: s.cssRG, css_ms: s.cssMS, css_ch: s.cssCH, css_ah: s.cssAH, css_is: s.cssIS, css_cc: s.cssCC, dss_dc: s.dssDC }
    }
}

/// Stack sizes for [`Pipeline::set_stack`] / [`Pipeline::set_stack_size`], in bytes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PipelineStack {
    /// Direct stack for direct callables called from intersection or any hit.
    pub direct_from_traversal: u32,
    /// Direct stack for direct callables called from raygen, miss or closest hit.
    pub direct_from_state: u32,
    /// Continuation stack.
    pub continuation: u32,
}

fn mul(a: u32, b: u32) -> Result<u32> {
    a.checked_mul(b).ok_or(Error::Invalid("stack size overflows u32"))
}

fn add(a: u32, b: u32) -> Result<u32> {
    a.checked_add(b).ok_or(Error::Invalid("stack size overflows u32"))
}

impl StackSizes {
    /// Element-wise maximum.
    pub fn max(self, o: Self) -> Self {
        Self {
            css_rg: self.css_rg.max(o.css_rg),
            css_ms: self.css_ms.max(o.css_ms),
            css_ch: self.css_ch.max(o.css_ch),
            css_ah: self.css_ah.max(o.css_ah),
            css_is: self.css_is.max(o.css_is),
            css_cc: self.css_cc.max(o.css_cc),
            dss_dc: self.dss_dc.max(o.dss_dc),
        }
    }

    /// Raises `self` to cover `group` (`optixUtilAccumulateStackSizes`); start from
    /// `StackSizes::default()`. See [`ProgramGroup::stack_sizes`] for `pipeline`.
    pub fn accumulate(&mut self, group: &ProgramGroup, pipeline: Option<&Pipeline>) -> Result<()> {
        *self = self.max(group.stack_sizes(pipeline)?);
        Ok(())
    }

    /// The common tail of the `optixUtilComputeStackSizes*` formulas: the continuation stack for
    /// a continuation-callable tree of `css_cc_tree` bytes.
    fn continuation(&self, css_cc_tree: u32, max_trace_depth: u32) -> Result<u32> {
        // Upper bound for a closest-hit or miss program plus its continuation-callable tree.
        let ch_or_ms = add(self.css_ch.max(self.css_ms), css_cc_tree)?;
        let nested = mul(max_trace_depth.max(1) - 1, ch_or_ms)?;
        let leaf = mul(max_trace_depth.min(1), ch_or_ms.max(add(self.css_is, self.css_ah)?))?;
        add(add(add(self.css_rg, css_cc_tree)?, nested)?, leaf)
    }

    /// `optixUtilComputeStackSizes`: stack sizes for trace recursion `max_trace_depth`,
    /// continuation-callable depth `max_cc_depth` and direct-callable depth `max_dc_depth`.
    pub fn compute(&self, max_trace_depth: u32, max_cc_depth: u32, max_dc_depth: u32) -> Result<PipelineStack> {
        let dc = mul(max_dc_depth, self.dss_dc)?;
        let continuation = self.continuation(mul(max_cc_depth, self.css_cc)?, max_trace_depth)?;
        Ok(PipelineStack { direct_from_traversal: dc, direct_from_state: dc, continuation })
    }

    /// `optixUtilComputeStackSizesDCSplit`: like [`StackSizes::compute`] with the direct
    /// callables' stack (`dss_dc_*`, replacing `self.dss_dc`) and depth split by call site.
    pub fn compute_dc_split(
        &self,
        dss_dc_from_traversal: u32,
        dss_dc_from_state: u32,
        max_trace_depth: u32,
        max_cc_depth: u32,
        max_dc_depth_from_traversal: u32,
        max_dc_depth_from_state: u32,
    ) -> Result<PipelineStack> {
        Ok(PipelineStack {
            direct_from_traversal: mul(max_dc_depth_from_traversal, dss_dc_from_traversal)?,
            direct_from_state: mul(max_dc_depth_from_state, dss_dc_from_state)?,
            continuation: self.continuation(mul(max_cc_depth, self.css_cc)?, max_trace_depth)?,
        })
    }

    /// `optixUtilComputeStackSizesCssCCTree`: like [`StackSizes::compute`] with the
    /// continuation stack of the whole continuation-callable tree given directly (replacing
    /// `self.css_cc` and the depth).
    pub fn compute_css_cc_tree(&self, css_cc_tree: u32, max_trace_depth: u32, max_dc_depth: u32) -> Result<PipelineStack> {
        let dc = mul(max_dc_depth, self.dss_dc)?;
        Ok(PipelineStack { direct_from_traversal: dc, direct_from_state: dc, continuation: self.continuation(css_cc_tree, max_trace_depth)? })
    }
}

/// `optixUtilComputeStackSizesSimplePathTracer`: stack sizes for camera rays (raygen `rg`, miss
/// `ms1`, closest hits `ch1`) whose closest hits trace shadow rays (miss `ms2`, closest hits
/// `ch2`), with no any-hit, intersection or callable programs.
pub fn simple_path_tracer(
    rg: &ProgramGroup,
    ms1: &ProgramGroup,
    ch1: &[Arc<ProgramGroup>],
    ms2: &ProgramGroup,
    ch2: &[Arc<ProgramGroup>],
    pipeline: Option<&Pipeline>,
) -> Result<PipelineStack> {
    let css_rg = rg.stack_sizes(pipeline)?.css_rg;
    let css_ms1 = ms1.stack_sizes(pipeline)?.css_ms;
    let css_ms2 = ms2.stack_sizes(pipeline)?.css_ms;
    let mut s1 = StackSizes::default();
    for g in ch1 {
        s1.accumulate(g, pipeline)?;
    }
    let mut s2 = StackSizes::default();
    for g in ch2 {
        s2.accumulate(g, pipeline)?;
    }
    let continuation = add(css_rg, css_ms1.max(add(s1.css_ch, css_ms2.max(s2.css_ch))?))?;
    Ok(PipelineStack { direct_from_traversal: 0, direct_from_state: 0, continuation })
}

impl ProgramGroup {
    /// The group's stack needs (`optixProgramGroupGetStackSize`). Programs calling external
    /// functions are only fully accounted with the `pipeline` they are linked into; without it
    /// OptiX leaves those out and warns.
    pub fn stack_sizes(&self, pipeline: Option<&Pipeline>) -> Result<StackSizes> {
        if let Some(p) = pipeline
            && !Arc::ptr_eq(p.context(), self.context())
        {
            return Err(Error::Invalid("pipeline from another device context"));
        }
        let mut s = sys::OptixStackSizes::default();
        // SAFETY: owned group handle; `pipeline` is null or a live pipeline of the same context;
        // `s` is writable.
        check("optixProgramGroupGetStackSize", unsafe {
            entry!(self.context().table(), optixProgramGroupGetStackSize)(self.raw(), &mut s, pipeline.map_or(std::ptr::null_mut(), Pipeline::raw))
        })?;
        Ok(s.into())
    }
}

impl Pipeline {
    /// Applies computed stack sizes with traversable graph depth `max_graph_depth`.
    pub fn set_stack(&self, stack: PipelineStack, max_graph_depth: u32) -> Result<()> {
        self.set_stack_size(stack.direct_from_traversal, stack.direct_from_state, stack.continuation, max_graph_depth)
    }
}
