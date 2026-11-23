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
//! workspace budget, and zero-allocation steady-state execution on a bound CUDA stream.

use std::ffi::c_void;
use std::ptr;
use std::sync::Arc;

use cuda_core::Context;
use cudnn_sys as sys;

use crate::{ConvMode, CudnnStatus, DataType, Error, Handle, Result, check, record_teardown};

const UID_X: i64 = 1;
const UID_W: i64 = 2;
const UID_Y: i64 = 3;

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

/// Heuristic search mode (`cudnnBackendHeurMode_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HeurMode {
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
    ctx: Arc<Context>,
}

impl BackendDesc {
    fn new(ctx: &Arc<Context>, desc_type: sys::cudnnBackendDescriptorType_t) -> Result<Self> {
        let mut raw: sys::cudnnBackendDescriptor_t = ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer; `cudnnBackendCreateDescriptor` allocates an opaque
        // descriptor destroyed in `Drop`.
        let status = unsafe { sys::cudnnBackendCreateDescriptor(desc_type, &mut raw) };
        check("cudnnBackendCreateDescriptor", status)?;
        Ok(Self { raw, ctx: Arc::clone(ctx) })
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
            record_teardown(&self.ctx, status);
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

/// Finalized cuDNN 9 Backend Execution Plan for a single 2D convolution (`fwd`, `bwd_data`, or
/// `bwd_filter`), reusable across steps with zero steady-state heap allocations.
pub struct GraphConvPlan {
    plan: BackendDesc,
    workspace_size: usize,
    ctx: Arc<Context>,
}

// SAFETY: `GraphConvPlan` is owned exclusively by a thread-local or mutex-guarded `CudnnRuntime`
// and all cuDNN calls serialize through `&mut Handle`.
unsafe impl Send for GraphConvPlan {}

impl GraphConvPlan {
    /// Builds and finalizes a cuDNN 9 Backend Execution Plan for `spec` under `max_workspace_bytes`.
    pub fn build(handle: &Handle, spec: &GraphConvSpec, max_workspace_bytes: usize) -> Result<Self> {
        let ctx = handle.context();
        let x_desc = make_tensor_desc(ctx, UID_X, spec.x_dtype, &spec.x_dims, &spec.x_strides, spec.alignment_bytes)?;
        let w_desc = make_tensor_desc(ctx, UID_W, spec.w_dtype, &spec.w_dims, &spec.w_strides, spec.alignment_bytes)?;
        let y_desc = make_tensor_desc(ctx, UID_Y, spec.y_dtype, &spec.y_dims, &spec.y_strides, spec.alignment_bytes)?;

        let mut conv_desc = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_CONVOLUTION_DESCRIPTOR)?;
        conv_desc.set_dtype(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_CONVOLUTION_COMP_TYPE, spec.compute_dtype)?;
        conv_desc.set_conv_mode(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_CONVOLUTION_CONV_MODE, ConvMode::CrossCorrelation)?;
        conv_desc.set_i64(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_CONVOLUTION_SPATIAL_DIMS, 2)?;
        conv_desc.set_i64_slice(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_CONVOLUTION_PRE_PADDINGS, &spec.pad)?;
        conv_desc.set_i64_slice(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_CONVOLUTION_POST_PADDINGS, &spec.pad)?;
        conv_desc.set_i64_slice(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_CONVOLUTION_DILATIONS, &spec.dilation)?;
        conv_desc.set_i64_slice(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_CONVOLUTION_FILTER_STRIDES, &spec.stride)?;
        conv_desc.finalize()?;

        let mut op = match spec.direction {
            GraphConvDirection::Forward => {
                let mut o = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_OPERATION_CONVOLUTION_FORWARD_DESCRIPTOR)?;
                o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_X, &x_desc)?;
                o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_W, &w_desc)?;
                o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_Y, &y_desc)?;
                o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_CONV_DESC, &conv_desc)?;
                o.set_f32(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_ALPHA, 1.0)?;
                o.set_f32(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_BETA, 0.0)?;
                o
            }
            GraphConvDirection::BackwardData => {
                let mut o = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_OPERATION_CONVOLUTION_BACKWARD_DATA_DESCRIPTOR)?;
                o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_DX, &x_desc)?;
                o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_W, &w_desc)?;
                o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_DY, &y_desc)?;
                o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_CONV_DESC, &conv_desc)?;
                o.set_f32(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_ALPHA, 1.0)?;
                o.set_f32(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_BETA, 0.0)?;
                o
            }
            GraphConvDirection::BackwardFilter => {
                let mut o = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_OPERATION_CONVOLUTION_BACKWARD_FILTER_DESCRIPTOR)?;
                o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_X, &x_desc)?;
                o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_DW, &w_desc)?;
                o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_DY, &y_desc)?;
                o.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_CONV_DESC, &conv_desc)?;
                o.set_f32(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_ALPHA, 1.0)?;
                o.set_f32(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_BETA, 0.0)?;
                o
            }
        };
        op.finalize()?;

        let mut op_graph = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_OPERATIONGRAPH_DESCRIPTOR)?;
        op_graph.set_handle(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATIONGRAPH_HANDLE, handle)?;
        op_graph.set_descs(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_OPERATIONGRAPH_OPS, &[op.raw])?;
        op_graph.finalize()?;

        let mut last_err: Option<Error> = None;
        for mode in [HeurMode::ModeA, HeurMode::ModeB, HeurMode::Fallback] {
            let mut heur = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_ENGINEHEUR_DESCRIPTOR)?;
            heur.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_ENGINEHEUR_OPERATION_GRAPH, &op_graph)?;
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
            let want = (total_cfgs as usize).min(16);
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
            for cfg in cfgs.iter().take(returned.max(0) as usize) {
                let mut plan = BackendDesc::new(ctx, sys::cudnnBackendDescriptorType_t_CUDNN_BACKEND_EXECUTION_PLAN_DESCRIPTOR)?;
                plan.set_handle(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_EXECUTION_PLAN_HANDLE, handle)?;
                plan.set_desc(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_EXECUTION_PLAN_ENGINE_CONFIG, cfg)?;
                if let Err(e) = plan.finalize() {
                    last_err = Some(e);
                    continue;
                }
                let ws_i64 = plan.get_i64(sys::cudnnBackendAttributeName_t_CUDNN_ATTR_EXECUTION_PLAN_WORKSPACE_SIZE)?;
                let ws = ws_i64.max(0) as usize;
                if ws <= max_workspace_bytes {
                    return Ok(Self {
                        plan,
                        workspace_size: ws,
                        ctx: Arc::clone(ctx),
                    });
                }
            }
        }
        Err(last_err.unwrap_or(Error::Status {
            call: "cudnn::graph::GraphConvPlan::build",
            code: CudnnStatus::NOT_SUPPORTED,
        }))
    }

    /// Required device workspace size in bytes for [`Self::execute_on_bound_stream`].
    #[must_use]
    pub const fn workspace_size(&self) -> usize {
        self.workspace_size
    }

    /// Executes the finalized plan on the stream currently bound to `handle`.
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
}
