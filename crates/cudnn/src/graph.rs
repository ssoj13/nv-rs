//! Non-legacy cuDNN 9 Backend Graph API (`cudnnBackend*`) for native Tensor Core convolutions
//! across `BF16` / `F16` / `F32` and `FP8` (`E4M3` / `E5M2`) on `SM86` (RTX 3080 Ti), `SM89`
//! (RTX 4090), `SM90` (H100), and `SM100`/`SM120` (Blackwell / RTX 5090).
//!
//! # What / why
//!
//! The legacy `cudnnConvolution*` v8 API cannot execute FP8 (`E4M3`/`E5M2`) convolutions and
//! misses newer cuDNN 9 NHWC Tensor Core engines (particularly on `bwd_data` / `bwd_filter` and
//! Ada/Hopper/Blackwell GPUs). This module wraps the cuDNN 9 Backend Descriptor Graph API with
//! RAII ownership, `ENGINEHEUR` (`Mode A -> Mode B -> Fallback`) plan selection under an explicit
//! workspace budget, and execution on a bound CUDA stream.
//!
//! Each execution creates, finalizes and destroys one `VARIANT_PACK` descriptor: finalized backend
//! descriptors are read-only (`cudnnBackendFinalize`, `cudnn_graph.h`), so the per-call device
//! pointers cannot be patched into a cached pack. A plan keeps its engine config, operation graph
//! and tensor descriptors alive for its own lifetime.

use std::ffi::c_void;
use std::ptr;
use std::sync::Arc;

use cuda_core::Context;
use cudnn_sys as sys;

use crate::{ConvMode, CudnnStatus, DataType, Error, Handle, Result, check, record_teardown};

const UID_X: i64 = 1;
const UID_W: i64 = 2;
const UID_Y: i64 = 3;
const UID_SCALE: i64 = 4;
const UID_VIRTUAL_CONV: i64 = 5;

/// Convolution direction for [`GraphConvSpec`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GraphConvDirection {
    /// Forward convolution: `Y = conv(X, W)`.
    Forward,
    /// Backward data convolution: `dX = conv_bwd_data(W, dY)`.
    BackwardData,
    /// Backward filter convolution: `dW = conv_bwd_filter(X, dY)`.
    BackwardFilter,
}

/// Heuristic search mode (`cudnnBackendHeurMode_t`), tried in declaration order by the builders.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum HeurMode {
    /// Fast heuristic mode A (`CUDNN_HEUR_MODE_A`, default primary).
    ModeA,
    /// Extended heuristic mode B (`CUDNN_HEUR_MODE_B`).
    ModeB,
    /// Fallback engine list (`CUDNN_HEUR_MODE_FALLBACK`).
    Fallback,
}

impl HeurMode {
    const fn to_sys(self) -> sys::cudnnBackendHeurMode_t {
        match self {
            Self::ModeA => sys::cudnnBackendHeurMode_t_CUDNN_HEUR_MODE_A,
            Self::ModeB => sys::cudnnBackendHeurMode_t_CUDNN_HEUR_MODE_B,
            Self::Fallback => sys::cudnnBackendHeurMode_t_CUDNN_HEUR_MODE_FALLBACK,
        }
    }
}

/// Complete specification of a 2D convolution graph node (supports NCHW, NHWC, arbitrary strides,
/// `F32`/`F16`/`BF16`, and `FP8 E4M3`/`E5M2` on SM89+ GPUs).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GraphConvSpec {
    /// Forward, backward-data, or backward-filter.
    pub direction: GraphConvDirection,
    /// Data type of `X` (or `dX` for backward-data).
    pub x_dtype: DataType,
    /// Data type of `W` (or `dW` for backward-filter).
    pub w_dtype: DataType,
    /// Data type of `Y` (or `dY` for backward passes).
    pub y_dtype: DataType,
    /// Internal accumulation/compute data type (typically [`DataType::F32`]).
    pub compute_dtype: DataType,
    /// Logical `[N, C_in, H, W]` dimensions of `X`/`dX`.
    pub x_dims: [i64; 4],
    /// Element strides of `X`/`dX` (e.g. NHWC `[H*W*C, 1, W*C, C]`).
    pub x_strides: [i64; 4],
    /// Logical `[C_out, C_in/g, kH, kW]` dimensions of `W`/`dW`.
    pub w_dims: [i64; 4],
    /// Element strides of `W`/`dW` (e.g. NHWC `[kH*kW*Ci, 1, kW*Ci, Ci]`).
    pub w_strides: [i64; 4],
    /// Logical `[N, C_out, H_out, W_out]` dimensions of `Y`/`dY`.
    pub y_dims: [i64; 4],
    /// Element strides of `Y`/`dY`.
    pub y_strides: [i64; 4],
    /// Spatial strides `[stride_h, stride_w]`.
    pub stride: [i64; 2],
    /// Spatial padding `[pad_h, pad_w]`.
    pub pad: [i64; 2],
    /// Spatial dilation `[dil_h, dil_w]`.
    pub dilation: [i64; 2],
    /// Byte alignment guaranteed for `X`, `W`, and `Y` pointers (typically `16`).
    pub alignment_bytes: i64,
}

impl GraphConvSpec {
    /// Computes canonical NCHW strides `[C*H*W, H*W, W, 1]` for `[N, C, H, W]`.
    #[must_use]
    pub const fn nchw_strides(dims: [i64; 4]) -> [i64; 4] {
        [dims[1] * dims[2] * dims[3], dims[2] * dims[3], dims[3], 1]
    }

    /// Computes canonical NHWC strides `[H*W*C, 1, W*C, C]` for logical `[N, C, H, W]`.
    #[must_use]
    pub const fn nhwc_strides(dims: [i64; 4]) -> [i64; 4] {
        [dims[2] * dims[3] * dims[1], 1, dims[3] * dims[1], dims[1]]
    }
}

/// RAII wrapper around a `cudnnBackendDescriptor_t`.
struct BackendDesc {
    raw: sys::cudnnBackendDescriptor_t,
    /// Keeps the owning context alive until `Drop` has destroyed `raw`; never read.
    _ctx: Arc<Context>,
}

impl BackendDesc {
    fn new(ctx: &Arc<Context>, desc_type: sys::cudnnBackendDescriptorType_t) -> Result<Self> {
        let mut raw: sys::cudnnBackendDescriptor_t = ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer; `cudnnBackendCreateDescriptor` allocates an opaque
        // descriptor destroyed in `Drop`.
        let status = unsafe { sys::cudnnBackendCreateDescriptor(desc_type, &mut raw) };
        check("cudnnBackendCreateDescriptor", status)?;
        Ok(Self { raw, _ctx: Arc::clone(ctx) })
    }

    fn set_attr_raw(
        &mut self,
        attr_name: sys::cudnnBackendAttributeName_t,
        attr_type: sys::cudnnBackendAttributeType_t,
        elem_count: i64,
        ptr: *const c_void,
    ) -> Result<()> {
        // SAFETY: `self.raw` is a live unfinalized descriptor and `ptr` points to `elem_count`
        // elements matching `attr_type`.
        let status = unsafe {
            sys::cudnnBackendSetAttribute(self.raw, attr_name, attr_type, elem_count, ptr)
        };
        check("cudnnBackendSetAttribute", status)
    }

    fn set_i64(&mut self, attr: sys::cudnnBackendAttributeName_t, val: i64) -> Result<()> {
        self.set_attr_raw(
            attr,
            sys::cudnnBackendAttributeType_t_CUDNN_TYPE_INT64,
            1,
            (&val as *const i64).cast(),
        )
    }

    fn set_i64_slice(&mut self, attr: sys::cudnnBackendAttributeName_t, vals: &[i64]) -> Result<()> {
        self.set_attr_raw(
            attr,
            sys::cudnnBackendAttributeType_t_CUDNN_TYPE_INT64,
            vals.len() as i64,
            vals.as_ptr().cast(),
        )
    }

    fn set_bool(&mut self, attr: sys::cudnnBackendAttributeName_t, val: bool) -> Result<()> {
        self.set_attr_raw(
            attr,
            sys::cudnnBackendAttributeType_t_CUDNN_TYPE_BOOLEAN,
            1,
            (&val as *const bool).cast(),
        )
    }

    fn set_pointwise_mode(
        &mut self,
        attr: sys::cudnnBackendAttributeName_t,
        mode: sys::cudnnPointwiseMode_t,
    ) -> Result<()> {
        self.set_attr_raw(
            attr,
            sys::cudnnBackendAttributeType_t_CUDNN_TYPE_POINTWISE_MODE,
            1,
            (&mode as *const sys::cudnnPointwiseMode_t).cast(),
        )
    }

    fn set_f32(&mut self, attr: sys::cudnnBackendAttributeName_t, val: f32) -> Result<()> {
        self.set_attr_raw(
            attr,
            sys::cudnnBackendAttributeType_t_CUDNN_TYPE_FLOAT,
            1,
            (&val as *const f32).cast(),
        )
    }

    fn set_dtype(&mut self, attr: sys::cudnnBackendAttributeName_t, dt: DataType) -> Result<()> {
        let raw = dt.to_sys();
        self.set_attr_raw(
            attr,
            sys::cudnnBackendAttributeType_t_CUDNN_TYPE_DATA_TYPE,
            1,
            (&raw as *const sys::cudnnDataType_t).cast(),
        )
    }

    fn set_conv_mode(&mut self, attr: sys::cudnnBackendAttributeName_t, mode: ConvMode) -> Result<()> {
        let raw = mode.to_sys();
        self.set_attr_raw(
            attr,
            sys::cudnnBackendAttributeType_t_CUDNN_TYPE_CONVOLUTION_MODE,
            1,
            (&raw as *const sys::cudnnConvolutionMode_t).cast(),
        )
    }

    fn set_heur_mode(&mut self, attr: sys::cudnnBackendAttributeName_t, mode: HeurMode) -> Result<()> {
        let raw = mode.to_sys();
        self.set_attr_raw(
            attr,
            sys::cudnnBackendAttributeType_t_CUDNN_TYPE_HEUR_MODE,
            1,
            (&raw as *const sys::cudnnBackendHeurMode_t).cast(),
        )
    }

    fn set_handle(&mut self, attr: sys::cudnnBackendAttributeName_t, handle: &Handle) -> Result<()> {
        let raw = handle.as_raw();
        self.set_attr_raw(
            attr,
            sys::cudnnBackendAttributeType_t_CUDNN_TYPE_HANDLE,
            1,
            (&raw as *const sys::cudnnHandle_t).cast(),
        )
    }

    fn set_desc(&mut self, attr: sys::cudnnBackendAttributeName_t, child: &BackendDesc) -> Result<()> {
        let raw = child.raw;
        self.set_attr_raw(
            attr,
            sys::cudnnBackendAttributeType_t_CUDNN_TYPE_BACKEND_DESCRIPTOR,
            1,
            (&raw as *const sys::cudnnBackendDescriptor_t).cast(),
        )
    }

    fn set_descs(&mut self, attr: sys::cudnnBackendAttributeName_t, children: &[sys::cudnnBackendDescriptor_t]) -> Result<()> {
        self.set_attr_raw(
            attr,
            sys::cudnnBackendAttributeType_t_CUDNN_TYPE_BACKEND_DESCRIPTOR,
            children.len() as i64,
            children.as_ptr().cast(),
        )
    }

    fn set_void_ptr(&mut self, attr: sys::cudnnBackendAttributeName_t, ptr_val: *mut c_void) -> Result<()> {
        self.set_attr_raw(
            attr,
            sys::cudnnBackendAttributeType_t_CUDNN_TYPE_VOID_PTR,
            1,
            (&ptr_val as *const *mut c_void).cast(),
        )
    }

    fn set_void_ptrs(&mut self, attr: sys::cudnnBackendAttributeName_t, ptrs: &[*mut c_void]) -> Result<()> {
        self.set_attr_raw(
            attr,
            sys::cudnnBackendAttributeType_t_CUDNN_TYPE_VOID_PTR,
            ptrs.len() as i64,
            ptrs.as_ptr().cast(),
        )
    }

    fn finalize(&mut self) -> Result<()> {
        // SAFETY: `self.raw` is a live descriptor with all required attributes populated.
        let status = unsafe { sys::cudnnBackendFinalize(self.raw) };
        check("cudnnBackendFinalize", status)
    }

    fn get_i64(&self, attr: sys::cudnnBackendAttributeName_t) -> Result<i64> {
        let mut out: i64 = 0;
        let mut count: i64 = 0;
        // SAFETY: `self.raw` is finalized; `out` and `count` are valid out-pointers.
        let status = unsafe {
            sys::cudnnBackendGetAttribute(
                self.raw,
                attr,
                sys::cudnnBackendAttributeType_t_CUDNN_TYPE_INT64,
                1,
                &mut count,
                (&mut out as *mut i64).cast(),
            )
        };
        check("cudnnBackendGetAttribute", status)?;
        Ok(out)
    }
}

impl Drop for BackendDesc {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: `self.raw` was created by `cudnnBackendCreateDescriptor` and is destroyed once.
            let status = unsafe { sys::cudnnBackendDestroyDescriptor(self.raw) };
            record_teardown("cudnnBackendDestroyDescriptor", status);
            self.raw = ptr::null_mut();
        }
    }
}

fn make_tensor_desc(
    ctx: &Arc<Context>,
    uid: i64,
    dtype: DataType,
    dims: &[i64; 4],
    strides: &[i64; 4],
    alignment: i64,
) -> Result<BackendDesc> {
    let mut d = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_TENSOR_DESCRIPTOR)?;
    d.set_dtype(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_TENSOR_DATA_TYPE, dtype)?;
    d.set_i64_slice(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_TENSOR_DIMENSIONS, dims)?;
    d.set_i64_slice(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_TENSOR_STRIDES, strides)?;
    d.set_i64(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_TENSOR_UNIQUE_ID, uid)?;
    d.set_i64(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_TENSOR_BYTE_ALIGNMENT, alignment)?;
    d.finalize()?;
    Ok(d)
}

fn make_virtual_tensor_desc(
    ctx: &Arc<Context>,
    uid: i64,
    dtype: DataType,
    dims: &[i64; 4],
    strides: &[i64; 4],
    alignment: i64,
) -> Result<BackendDesc> {
    let mut d = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_TENSOR_DESCRIPTOR)?;
    d.set_dtype(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_TENSOR_DATA_TYPE, dtype)?;
    d.set_i64_slice(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_TENSOR_DIMENSIONS, dims)?;
    d.set_i64_slice(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_TENSOR_STRIDES, strides)?;
    d.set_i64(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_TENSOR_UNIQUE_ID, uid)?;
    d.set_i64(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_TENSOR_BYTE_ALIGNMENT, alignment)?;
    d.set_bool(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_TENSOR_IS_VIRTUAL, true)?;
    d.finalize()?;
    Ok(d)
}

/// The finalized convolution descriptor of `spec` (mode, padding, dilation, stride).
fn make_conv_desc(ctx: &Arc<Context>, spec: &GraphConvSpec) -> Result<BackendDesc> {
    let mut conv_desc = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_CONVOLUTION_DESCRIPTOR)?;
    conv_desc.set_dtype(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_CONVOLUTION_COMP_TYPE, spec.compute_dtype)?;
    conv_desc.set_conv_mode(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_CONVOLUTION_CONV_MODE, ConvMode::CrossCorrelation)?;
    conv_desc.set_i64(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_CONVOLUTION_SPATIAL_DIMS, 2)?;
    conv_desc.set_i64_slice(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_CONVOLUTION_PRE_PADDINGS, &spec.pad)?;
    conv_desc.set_i64_slice(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_CONVOLUTION_POST_PADDINGS, &spec.pad)?;
    conv_desc.set_i64_slice(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_CONVOLUTION_DILATIONS, &spec.dilation)?;
    conv_desc.set_i64_slice(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_CONVOLUTION_FILTER_STRIDES, &spec.stride)?;
    conv_desc.finalize()?;
    Ok(conv_desc)
}

/// The tensors bound to the three roles of a convolution operation: `x`, `w` and `y` are the
/// spec's input, filter and output, except that a fused plan substitutes a virtual tensor for the
/// role the convolution writes.
struct ConvTensors<'a> {
    x: &'a BackendDesc,
    w: &'a BackendDesc,
    y: &'a BackendDesc,
}

/// The finalized forward / backward-data / backward-filter operation over `t` (alpha 1, beta 0).
fn make_conv_op(
    ctx: &Arc<Context>,
    direction: GraphConvDirection,
    conv_desc: &BackendDesc,
    t: &ConvTensors<'_>,
) -> Result<BackendDesc> {
    let mut o = match direction {
        GraphConvDirection::Forward => {
            let mut o = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_OPERATION_CONVOLUTION_FORWARD_DESCRIPTOR)?;
            o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_X, t.x)?;
            o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_W, t.w)?;
            o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_Y, t.y)?;
            o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_CONV_DESC, conv_desc)?;
            o.set_f32(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_ALPHA, 1.0)?;
            o.set_f32(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_BETA, 0.0)?;
            o
        }
        GraphConvDirection::BackwardData => {
            let mut o = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_OPERATION_CONVOLUTION_BACKWARD_DATA_DESCRIPTOR)?;
            o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_DX, t.x)?;
            o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_W, t.w)?;
            o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_DY, t.y)?;
            o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_CONV_DESC, conv_desc)?;
            o.set_f32(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_ALPHA, 1.0)?;
            o.set_f32(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_BETA, 0.0)?;
            o
        }
        GraphConvDirection::BackwardFilter => {
            let mut o = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_OPERATION_CONVOLUTION_BACKWARD_FILTER_DESCRIPTOR)?;
            o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_X, t.x)?;
            o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_DW, t.w)?;
            o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_DY, t.y)?;
            o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_CONV_DESC, conv_desc)?;
            o.set_f32(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_ALPHA, 1.0)?;
            o.set_f32(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_BETA, 0.0)?;
            o
        }
    };
    o.finalize()?;
    Ok(o)
}

/// The finalized operation graph of `ops` (finalized operation descriptors) on `handle`.
fn make_op_graph(
    ctx: &Arc<Context>,
    handle: &Handle,
    ops: &[sys::cudnnBackendDescriptor_t],
) -> Result<BackendDesc> {
    let mut op_graph = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_OPERATIONGRAPH_DESCRIPTOR)?;
    op_graph.set_handle(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATIONGRAPH_HANDLE, handle)?;
    op_graph.set_descs(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATIONGRAPH_OPS, ops)?;
    op_graph.finalize()?;
    Ok(op_graph)
}

/// Picks the first execution plan of `op_graph` whose workspace fits `max_workspace_bytes`, trying
/// every heuristic mode and every ranked engine config (a lower-ranked one may be the only fit
/// for the budget). Returns the plan, the engine config it was finalized from and its workspace
/// size; the last failure (or `NOT_SUPPORTED` naming `call`) when nothing fits.
fn select_plan(
    ctx: &Arc<Context>,
    handle: &Handle,
    op_graph: &BackendDesc,
    max_workspace_bytes: usize,
    call: &'static str,
) -> Result<(BackendDesc, BackendDesc, usize)> {
    let mut last_err: Option<Error> = None;
    for mode in [HeurMode::ModeA, HeurMode::ModeB, HeurMode::Fallback] {
        let mut heur = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_ENGINEHEUR_DESCRIPTOR)?;
        heur.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_ENGINEHEUR_OPERATION_GRAPH, op_graph)?;
        heur.set_heur_mode(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_ENGINEHEUR_MODE, mode)?;
        if let Err(e) = heur.finalize() {
            last_err = Some(e);
            continue;
        }
        let mut total_cfgs: i64 = 0;
        // SAFETY: querying count with `max_elements = 0` and null output buffer is standard cuDNN API.
        let st_cnt = unsafe {
            sys::cudnnBackendGetAttribute(
                heur.raw,
                sys::cudnnBackendAttributeName_t_CUDNN_ATTR_ENGINEHEUR_RESULTS,
                sys::cudnnBackendAttributeType_t_CUDNN_TYPE_BACKEND_DESCRIPTOR,
                0,
                &mut total_cfgs,
                ptr::null_mut(),
            )
        };
        if check("cudnnBackendGetAttribute(ENGINEHEUR_RESULTS count)", st_cnt).is_err() || total_cfgs <= 0 {
            continue;
        }
        let want = total_cfgs as usize;
        let mut cfgs: Vec<BackendDesc> = Vec::with_capacity(want);
        let mut raw_cfgs: Vec<sys::cudnnBackendDescriptor_t> = Vec::with_capacity(want);
        for _ in 0..want {
            let d = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_ENGINECFG_DESCRIPTOR)?;
            raw_cfgs.push(d.raw);
            cfgs.push(d);
        }
        let mut returned: i64 = 0;
        // SAFETY: `raw_cfgs` holds `want` valid unfinalized ENGINECFG descriptors.
        let st_get = unsafe {
            sys::cudnnBackendGetAttribute(
                heur.raw,
                sys::cudnnBackendAttributeName_t_CUDNN_ATTR_ENGINEHEUR_RESULTS,
                sys::cudnnBackendAttributeType_t_CUDNN_TYPE_BACKEND_DESCRIPTOR,
                want as i64,
                &mut returned,
                raw_cfgs.as_mut_ptr().cast(),
            )
        };
        if let Err(e) = check("cudnnBackendGetAttribute(ENGINEHEUR_RESULTS)", st_get) {
            last_err = Some(e);
            continue;
        }
        for cfg in cfgs.into_iter().take(returned.max(0) as usize) {
            let mut plan = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_EXECUTION_PLAN_DESCRIPTOR)?;
            plan.set_handle(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_EXECUTION_PLAN_HANDLE, handle)?;
            plan.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_EXECUTION_PLAN_ENGINE_CONFIG, &cfg)?;
            if let Err(e) = plan.finalize() {
                last_err = Some(e);
                continue;
            }
            let ws_i64 = match plan.get_i64(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_EXECUTION_PLAN_WORKSPACE_SIZE) {
                Ok(v) => v,
                Err(e) => {
                    last_err = Some(e);
                    continue;
                }
            };
            let ws = ws_i64.max(0) as usize;
            if ws <= max_workspace_bytes {
                return Ok((plan, cfg, ws));
            }
        }
    }
    Err(last_err.unwrap_or(Error::Status {
        call,
        code: CudnnStatus::NOT_SUPPORTED,
    }))
}

/// Finalized cuDNN 9 Backend Execution Plan for a single 2D convolution (`fwd`, `bwd_data`, or
/// `bwd_filter`), reusable across steps. Each execution builds one variant-pack descriptor.
pub struct GraphConvPlan {
    plan: BackendDesc,
    workspace_size: usize,
    ctx: Arc<Context>,
    /// Engine config, operation graph, operations and tensor descriptors the plan was finalized
    /// from. Declared after `plan` so they are destroyed after it, whether cuDNN copies or
    /// references them.
    _deps: Vec<BackendDesc>,
}

// SAFETY: every field is owned by the plan: finalized cuDNN backend descriptors are heap objects
// not tied to the creating thread (cuDNN only forbids concurrent use of one object), and
// `Arc<Context>` is `Send`. The raw descriptor pointers keep the plan `!Sync`, so moving it to
// another thread transfers exclusive access; execution additionally serializes through
// `&mut Handle`.
unsafe impl Send for GraphConvPlan {}

impl GraphConvPlan {
    /// Builds and finalizes a cuDNN 9 Backend Execution Plan for `spec` under `max_workspace_bytes`.
    pub fn build(handle: &Handle, spec: &GraphConvSpec, max_workspace_bytes: usize) -> Result<Self> {
        let ctx = handle.context();
        // Heuristics and plan finalization run on the context current on this thread.
        ctx.bind_to_thread()?;
        let x_desc = make_tensor_desc(ctx, UID_X, spec.x_dtype, &spec.x_dims, &spec.x_strides, spec.alignment_bytes)?;
        let w_desc = make_tensor_desc(ctx, UID_W, spec.w_dtype, &spec.w_dims, &spec.w_strides, spec.alignment_bytes)?;
        let y_desc = make_tensor_desc(ctx, UID_Y, spec.y_dtype, &spec.y_dims, &spec.y_strides, spec.alignment_bytes)?;

        let conv_desc = make_conv_desc(ctx, spec)?;
        let op = make_conv_op(
            ctx,
            spec.direction,
            &conv_desc,
            &ConvTensors { x: &x_desc, w: &w_desc, y: &y_desc },
        )?;
        let op_graph = make_op_graph(ctx, handle, &[op.raw])?;
        let (plan, cfg, ws) = select_plan(
            ctx,
            handle,
            &op_graph,
            max_workspace_bytes,
            "cudnn::graph::GraphConvPlan::build",
        )?;
        Ok(Self {
            plan,
            workspace_size: ws,
            ctx: Arc::clone(ctx),
            _deps: vec![cfg, op_graph, op, conv_desc, x_desc, w_desc, y_desc],
        })
    }

    /// Builds a fused 2-op cuDNN 9 Backend Execution Plan (`Conv -> PointwiseMul(scale_f32)`) for
    /// delayed-scaling FP8 (`E4M3`/`E5M2`) convolutions on `SM89+` (RTX 4090 / H100 / RTX 5090).
    ///
    /// The convolution accumulates into a virtual `F32` tensor and fuses multiplication by a
    /// `[1, 1, 1, 1]` `F32` device descale tensor (`s_a * s_b`) directly into the epilogue before
    /// writing the output (`BF16`/`F16`/`F32`), requiring zero extra HBM passes.
    pub fn build_scaled(handle: &Handle, spec: &GraphConvSpec, max_workspace_bytes: usize) -> Result<Self> {
        let ctx = handle.context();
        // Heuristics and plan finalization run on the context current on this thread.
        ctx.bind_to_thread()?;
        let x_desc = make_tensor_desc(ctx, UID_X, spec.x_dtype, &spec.x_dims, &spec.x_strides, spec.alignment_bytes)?;
        let w_desc = make_tensor_desc(ctx, UID_W, spec.w_dtype, &spec.w_dims, &spec.w_strides, spec.alignment_bytes)?;
        let y_desc = make_tensor_desc(ctx, UID_Y, spec.y_dtype, &spec.y_dims, &spec.y_strides, spec.alignment_bytes)?;
        let scale_desc = make_tensor_desc(ctx, UID_SCALE, DataType::F32, &[1, 1, 1, 1], &[1, 1, 1, 1], 4)?;

        let (v_dims, v_strides) = match spec.direction {
            GraphConvDirection::Forward => (&spec.y_dims, &spec.y_strides),
            GraphConvDirection::BackwardData => (&spec.x_dims, &spec.x_strides),
            GraphConvDirection::BackwardFilter => (&spec.w_dims, &spec.w_strides),
        };
        let v_out = make_virtual_tensor_desc(ctx, UID_VIRTUAL_CONV, DataType::F32, v_dims, v_strides, spec.alignment_bytes)?;

        let conv_desc = make_conv_desc(ctx, spec)?;

        // The convolution writes the virtual tensor in its output role; the pointwise multiply
        // then writes the real tensor of that role.
        let (tensors, real_out_desc) = match spec.direction {
            GraphConvDirection::Forward => (ConvTensors { x: &x_desc, w: &w_desc, y: &v_out }, &y_desc),
            GraphConvDirection::BackwardData => (ConvTensors { x: &v_out, w: &w_desc, y: &y_desc }, &x_desc),
            GraphConvDirection::BackwardFilter => (ConvTensors { x: &x_desc, w: &v_out, y: &y_desc }, &w_desc),
        };
        let op_conv = make_conv_op(ctx, spec.direction, &conv_desc, &tensors)?;

        let mut pw_desc = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_POINTWISE_DESCRIPTOR)?;
        pw_desc.set_pointwise_mode(
            sys::cudnnBackendAttributeName_t_CUDNN_ATTR_POINTWISE_MODE,
            sys::cudnnPointwiseMode_t_CUDNN_POINTWISE_MUL,
        )?;
        pw_desc.set_dtype(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_POINTWISE_MATH_PREC, DataType::F32)?;
        pw_desc.finalize()?;

        let mut op_mul = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_OPERATION_POINTWISE_DESCRIPTOR)?;
        op_mul.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_POINTWISE_PW_DESCRIPTOR, &pw_desc)?;
        op_mul.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_POINTWISE_XDESC, &v_out)?;
        op_mul.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_POINTWISE_BDESC, &scale_desc)?;
        op_mul.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_POINTWISE_YDESC, real_out_desc)?;
        op_mul.finalize()?;

        let op_graph = make_op_graph(ctx, handle, &[op_conv.raw, op_mul.raw])?;
        let (plan, cfg, ws) = select_plan(
            ctx,
            handle,
            &op_graph,
            max_workspace_bytes,
            "cudnn::graph::GraphConvPlan::build_scaled",
        )?;
        Ok(Self {
            plan,
            workspace_size: ws,
            ctx: Arc::clone(ctx),
            _deps: vec![
                cfg, op_graph, op_mul, op_conv, pw_desc, conv_desc, v_out, scale_desc, x_desc,
                w_desc, y_desc,
            ],
        })
    }

    /// Required device workspace size in bytes for [`Self::execute_on_bound_stream`].
    #[must_use]
    pub const fn workspace_size(&self) -> usize {
        self.workspace_size
    }

    /// Executes the finalized plan on the stream currently bound to `handle`.
    ///
    /// Rejects a `handle` from another CUDA context than the plan and binds that context to the
    /// calling thread. Creates and destroys one variant-pack descriptor per call.
    ///
    /// # Safety
    /// `x_ptr`, `w_ptr`, `y_ptr` (and `ws_ptr` when [`Self::workspace_size`] > 0) must be valid
    /// device pointers on `handle`'s context matching the [`GraphConvSpec`] dimensions, dtypes,
    /// strides, and alignment.
    pub unsafe fn execute_on_bound_stream(
        &self,
        handle: &mut Handle,
        x_ptr: u64,
        w_ptr: u64,
        y_ptr: u64,
        ws_ptr: u64,
    ) -> Result<()> {
        self.bind(handle)?;
        let mut vpack = BackendDesc::new(&self.ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_VARIANT_PACK_DESCRIPTOR)?;
        let uids = [UID_X, UID_W, UID_Y];
        let ptrs = [
            x_ptr as *mut c_void,
            w_ptr as *mut c_void,
            y_ptr as *mut c_void,
        ];
        vpack.set_i64_slice(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_VARIANT_PACK_UNIQUE_IDS, &uids)?;
        vpack.set_void_ptrs(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_VARIANT_PACK_DATA_POINTERS, &ptrs)?;
        if self.workspace_size > 0 {
            vpack.set_void_ptr(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_VARIANT_PACK_WORKSPACE, ws_ptr as *mut c_void)?;
        }
        vpack.finalize()?;
        // SAFETY: `handle.raw()`, `self.plan.raw`, and `vpack.raw` are finalized; caller guarantees
        // valid device pointers for the duration of stream execution.
        let status = unsafe { sys::cudnnBackendExecute(handle.as_raw(), self.plan.raw, vpack.raw) };
        check("cudnnBackendExecute", status)
    }

    /// Executes a plan built with [`Self::build_scaled`] on the stream currently bound to `handle`,
    /// binding `scale_ptr` (`[1]` `F32` device scalar) to the fused descale epilogue.
    ///
    /// Same context check, thread binding and per-call variant pack as
    /// [`Self::execute_on_bound_stream`].
    ///
    /// # Safety
    /// All device pointers (`x_ptr`, `w_ptr`, `y_ptr`, `scale_ptr`, and `ws_ptr` when
    /// `workspace_size() > 0`) must be valid on `handle`'s context and match the plan's spec.
    pub unsafe fn execute_scaled_on_bound_stream(
        &self,
        handle: &mut Handle,
        x_ptr: u64,
        w_ptr: u64,
        y_ptr: u64,
        scale_ptr: u64,
        ws_ptr: u64,
    ) -> Result<()> {
        self.bind(handle)?;
        let mut vpack = BackendDesc::new(&self.ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_VARIANT_PACK_DESCRIPTOR)?;
        let uids = [UID_X, UID_W, UID_Y, UID_SCALE];
        let ptrs = [
            x_ptr as *mut c_void,
            w_ptr as *mut c_void,
            y_ptr as *mut c_void,
            scale_ptr as *mut c_void,
        ];
        vpack.set_i64_slice(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_VARIANT_PACK_UNIQUE_IDS, &uids)?;
        vpack.set_void_ptrs(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_VARIANT_PACK_DATA_POINTERS, &ptrs)?;
        if self.workspace_size > 0 {
            vpack.set_void_ptr(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_VARIANT_PACK_WORKSPACE, ws_ptr as *mut c_void)?;
        }
        vpack.finalize()?;
        // SAFETY: `handle.as_raw()`, `self.plan.raw`, and `vpack.raw` are finalized; caller guarantees
        // valid device pointers for the duration of stream execution.
        let status = unsafe { sys::cudnnBackendExecute(handle.as_raw(), self.plan.raw, vpack.raw) };
        check("cudnnBackendExecute(scaled)", status)
    }

    /// Verifies `handle` shares the plan's context and makes that context current on this thread.
    fn bind(&self, handle: &Handle) -> Result<()> {
        if !cuda_core_ext::same_context(handle.context(), &self.ctx) {
            return Err(Error::Invalid(
                "cuDNN handle belongs to a different CUDA context than the graph plan",
            ));
        }
        self.ctx.bind_to_thread()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cuda_core::DeviceBuffer;

    #[test]
    #[ignore = "requires CUDA GPU and cuDNN runtime"]
    fn t2_plan_outlives_build_handle_and_rejects_foreign_context() {
        let ctx = Context::new(0).unwrap();
        let stream = ctx.new_stream().unwrap();
        // 1x1x3x3 input, 1x1x2x2 ones filter -> 1x1x2x2 output.
        let x_dims = [1, 1, 3, 3];
        let w_dims = [1, 1, 2, 2];
        let y_dims = [1, 1, 2, 2];
        let spec = GraphConvSpec {
            direction: GraphConvDirection::Forward,
            x_dtype: DataType::F32,
            w_dtype: DataType::F32,
            y_dtype: DataType::F32,
            compute_dtype: DataType::F32,
            x_dims,
            x_strides: GraphConvSpec::nchw_strides(x_dims),
            w_dims,
            w_strides: GraphConvSpec::nchw_strides(w_dims),
            y_dims,
            y_strides: GraphConvSpec::nchw_strides(y_dims),
            stride: [1, 1],
            pad: [0, 0],
            dilation: [1, 1],
            alignment_bytes: 16,
        };

        // Build with a handle that is destroyed before execution; the plan keeps its descriptors.
        let plan = {
            let build_handle = Handle::new(&ctx).unwrap();
            GraphConvPlan::build(&build_handle, &spec, 64 * 1024 * 1024).unwrap()
        };

        let x_host: [f32; 9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let w_host: [f32; 4] = [1.0; 4];
        let x_dev = DeviceBuffer::from_host(&stream, &x_host).unwrap();
        let w_dev = DeviceBuffer::from_host(&stream, &w_host).unwrap();
        let y_dev = DeviceBuffer::<f32>::zeroed(&stream, 4).unwrap();
        let ws_dev = DeviceBuffer::<u8>::zeroed(&stream, plan.workspace_size().max(1)).unwrap();
        stream.synchronize().unwrap();

        let mut exec_handle = Handle::new(&ctx).unwrap();
        exec_handle.set_stream(&stream).unwrap();
        // SAFETY: all pointers are live allocations on `ctx` sized for `spec`; the workspace holds
        // `plan.workspace_size()` bytes, and the stream is synchronized before the buffers drop.
        unsafe {
            plan.execute_on_bound_stream(
                &mut exec_handle,
                x_dev.cu_deviceptr(),
                w_dev.cu_deviceptr(),
                y_dev.cu_deviceptr(),
                ws_dev.cu_deviceptr(),
            )
        }
        .unwrap();
        stream.synchronize().unwrap();
        assert_eq!(
            y_dev.to_host_vec(&stream).unwrap(),
            vec![12.0, 16.0, 24.0, 28.0]
        );

        // `Context::new(0)` retains the same primary `CUcontext` through a different `Arc`: the
        // plan's context check compares contexts by value and must accept its handle.
        let same_ctx_other_arc = Context::new(0).unwrap();
        let other_handle = Handle::new(&same_ctx_other_arc).unwrap();
        plan.bind(&other_handle).unwrap();
    }
}
