//! Safe NVIDIA Optical Flow (NVOFA) CUDA wrapper over `nvofa-sys` and `cuda-core`.
//!
//! **What.**
//! - [`OpticalFlowSession`] — opens a CUDA optical-flow handle (`nvCreateOpticalFlowCuda`),
//!   queries hardware capabilities (`nvOFGetCaps`), initializes the session (`nvOFInit` with
//!   output/hint grid sizes, cost output, and ROI support), binds and retains input/output CUDA
//!   streams, and executes flow estimation (`nvOFExecute`) with zero per-frame allocations after
//!   checking every buffer against the `InitConfig`. A mutex serializes all `&self` calls into
//!   the session handle, which the Optical Flow SDK does not document as thread-safe.
//! - [`OfBuffer`] — wraps `NvOFGPUBufferHandle` (`CUdeviceptr` or `CUarray`), queries plane
//!   strides (`nvOFGPUBufferGetStrideInfo`), and provides zero-allocation 2D upload/download and
//!   flow-vector readback; these first wait for the session's I/O streams (or the context when
//!   none are bound), because the caller's copy stream is not ordered against `nvOFExecute`.
//! - [`FlowVector`] — S10.5 fixed-point `(flow_x, flow_y)` vector with pixel conversion.
//! - [`RoiRect`] — Region-of-Interest rectangle with hardware alignment validation.
//!
//! **Conventions.** Every handle owns its NVOFA object, holds an `Arc` of its parent, and destroys
//! it on `Drop`. A failing NVOFA teardown call is kept with its real `NV_OF_STATUS` in
//! `cuda_core_ext::teardown` (`Library::Nvofa`); a failed context bind stays on the CUDA context
//! (`cuda_core::Context::record_err`).
//! **Used by** motion-vector pipelines, frame interpolation, and the NVOFA T3 GPU tests.

pub use nvofa_sys as sys;

use cuda_core::{Context, DriverError, Stream};
use cuda_core_ext::media;
use cuda_core_ext::teardown::{self, Library};
use std::ffi::CStr;
use std::fmt;
use std::mem::zeroed;
use std::sync::{Arc, Mutex};

/// Raw `NV_OF_STATUS` code wrapped as a `Copy` integer newtype.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct StatusCode(pub sys::NV_OF_STATUS);

impl StatusCode {
    /// Returns the underlying integer status code.
    pub const fn raw(self) -> sys::NV_OF_STATUS {
        self.0
    }
}

impl fmt::Debug for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StatusCode({})", self.0)
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NVOFA status {}", self.0)
    }
}

/// Everything an NVOFA call can fail with.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// `nvofapi64` could not be loaded or is missing an entry point.
    #[error("NVOFA unavailable: {0}")]
    Unavailable(String),
    /// An NVOFA entry point returned a non-success status.
    #[error("{call} failed with {code}{}", media::detail_suffix(.detail))]
    Status {
        /// The NVOFA function-list entry point.
        call: &'static str,
        /// The raw `NV_OF_STATUS` code.
        code: StatusCode,
        /// Optional message from `nvOFGetLastError`.
        detail: String,
    },
    /// A CUDA driver call failed.
    #[error("CUDA: {0}")]
    Driver(#[from] DriverError),
    /// Hardware does not support the requested grid size, mode, or feature.
    #[error("unsupported {what}: required {required}, found {found}")]
    Unsupported {
        /// What was requested.
        what: &'static str,
        /// Required value.
        required: u32,
        /// Reported value.
        found: u32,
    },
    /// An argument was rejected before reaching NVOFA.
    #[error("invalid argument: {0}")]
    Invalid(&'static str),
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

fn fn_list() -> Result<&'static sys::NV_OF_CUDA_API_FUNCTION_LIST> {
    sys::api()
        .map(|a| &a.list)
        .map_err(|e| Error::Unavailable(e.to_string()))
}

macro_rules! entry {
    ($list:expr, $name:ident) => {
        $list.$name.ok_or_else(|| {
            Error::Unavailable(
                concat!(stringify!($name), " missing from NVOFA function list").into(),
            )
        })?
    };
}

fn check(
    call: &'static str,
    status: sys::NV_OF_STATUS,
    handle: Option<sys::NvOFHandle>,
    list: &sys::NV_OF_CUDA_API_FUNCTION_LIST,
) -> Result<()> {
    if status == sys::_NV_OF_STATUS_NV_OF_SUCCESS {
        return Ok(());
    }
    let mut detail = String::new();
    if let (Some(h), Some(get_err)) = (handle, list.nvOFGetLastError)
        && !h.is_null()
    {
        let mut buf = [0i8; 256];
        let mut size = buf.len() as u32;
        // SAFETY: `h` is a live NVOFA handle and `buf` has `size` writable bytes.
        let rc = unsafe { get_err(h, buf.as_mut_ptr(), &mut size) };
        if rc == sys::_NV_OF_STATUS_NV_OF_SUCCESS {
            // SAFETY: `nvOFGetLastError` NUL-terminates `buf`.
            detail = unsafe { CStr::from_ptr(buf.as_ptr()) }
                .to_string_lossy()
                .trim()
                .to_owned();
        }
    }
    Err(Error::Status {
        call,
        code: StatusCode(status),
        detail,
    })
}

/// Output flow-vector grid size (`NV_OF_OUTPUT_VECTOR_GRID_SIZE`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GridSize {
    /// 1x1 pixel per flow vector.
    One = 1,
    /// 2x2 pixels per flow vector.
    Two = 2,
    /// 4x4 pixels per flow vector.
    Four = 4,
}

impl GridSize {
    const fn to_sys(self) -> sys::NV_OF_OUTPUT_VECTOR_GRID_SIZE {
        match self {
            Self::One => sys::_NV_OF_OUTPUT_VECTOR_GRID_SIZE_NV_OF_OUTPUT_VECTOR_GRID_SIZE_1,
            Self::Two => sys::_NV_OF_OUTPUT_VECTOR_GRID_SIZE_NV_OF_OUTPUT_VECTOR_GRID_SIZE_2,
            Self::Four => sys::_NV_OF_OUTPUT_VECTOR_GRID_SIZE_NV_OF_OUTPUT_VECTOR_GRID_SIZE_4,
        }
    }

    /// Computes output grid dimensions `(grid_w, grid_h)` for a `width x height` frame.
    pub const fn grid_dims(self, width: u32, height: u32) -> (u32, u32) {
        let g = self as u32;
        (width.div_ceil(g), height.div_ceil(g))
    }
}

/// External hint vector grid size (`NV_OF_HINT_VECTOR_GRID_SIZE`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum HintGridSize {
    /// 1x1 pixel per hint vector.
    One = 1,
    /// 2x2 pixels per hint vector.
    Two = 2,
    /// 4x4 pixels per hint vector.
    Four = 4,
    /// 8x8 pixels per hint vector.
    Eight = 8,
}

impl HintGridSize {
    const fn to_sys(self) -> sys::NV_OF_HINT_VECTOR_GRID_SIZE {
        match self {
            Self::One => sys::_NV_OF_HINT_VECTOR_GRID_SIZE_NV_OF_HINT_VECTOR_GRID_SIZE_1,
            Self::Two => sys::_NV_OF_HINT_VECTOR_GRID_SIZE_NV_OF_HINT_VECTOR_GRID_SIZE_2,
            Self::Four => sys::_NV_OF_HINT_VECTOR_GRID_SIZE_NV_OF_HINT_VECTOR_GRID_SIZE_4,
            Self::Eight => sys::_NV_OF_HINT_VECTOR_GRID_SIZE_NV_OF_HINT_VECTOR_GRID_SIZE_8,
        }
    }
}

/// Performance/quality preset (`NV_OF_PERF_LEVEL`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PerfLevel {
    /// Highest quality, lowest speed.
    #[default]
    Slow,
    /// Balanced speed and quality.
    Medium,
    /// Highest speed, lower quality.
    Fast,
}

impl PerfLevel {
    const fn to_sys(self) -> sys::NV_OF_PERF_LEVEL {
        match self {
            Self::Slow => sys::_NV_OF_PERF_LEVEL_NV_OF_PERF_LEVEL_SLOW,
            Self::Medium => sys::_NV_OF_PERF_LEVEL_NV_OF_PERF_LEVEL_MEDIUM,
            Self::Fast => sys::_NV_OF_PERF_LEVEL_NV_OF_PERF_LEVEL_FAST,
        }
    }
}

/// Buffer usage (`NV_OF_BUFFER_USAGE`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BufferUsage {
    /// Input or reference frame.
    Input,
    /// Output flow vector or disparity buffer.
    Output,
    /// External hint buffer.
    Hint,
    /// Output cost buffer.
    Cost,
}

impl BufferUsage {
    const fn to_sys(self) -> sys::NV_OF_BUFFER_USAGE {
        match self {
            Self::Input => sys::_NV_OF_BUFFER_USAGE_NV_OF_BUFFER_USAGE_INPUT,
            Self::Output => sys::_NV_OF_BUFFER_USAGE_NV_OF_BUFFER_USAGE_OUTPUT,
            Self::Hint => sys::_NV_OF_BUFFER_USAGE_NV_OF_BUFFER_USAGE_HINT,
            Self::Cost => sys::_NV_OF_BUFFER_USAGE_NV_OF_BUFFER_USAGE_COST,
        }
    }
}

/// Buffer pixel/element format (`NV_OF_BUFFER_FORMAT`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BufferFormat {
    /// 8-bit grayscale input.
    Grayscale8,
    /// 8-bit semi-planar NV12 input.
    Nv12,
    /// 8-bit packed ABGR8 input.
    Abgr8,
    /// 16-bit signed short (stereo disparity).
    Short,
    /// 2x16-bit signed short (optical flow vector S10.5).
    Short2,
    /// 8-bit unsigned cost.
    Uint8,
    /// 32-bit unsigned legacy cost.
    Uint,
}

impl BufferFormat {
    const fn to_sys(self) -> sys::NV_OF_BUFFER_FORMAT {
        match self {
            Self::Grayscale8 => sys::_NV_OF_BUFFER_FORMAT_NV_OF_BUFFER_FORMAT_GRAYSCALE8,
            Self::Nv12 => sys::_NV_OF_BUFFER_FORMAT_NV_OF_BUFFER_FORMAT_NV12,
            Self::Abgr8 => sys::_NV_OF_BUFFER_FORMAT_NV_OF_BUFFER_FORMAT_ABGR8,
            Self::Short => sys::_NV_OF_BUFFER_FORMAT_NV_OF_BUFFER_FORMAT_SHORT,
            Self::Short2 => sys::_NV_OF_BUFFER_FORMAT_NV_OF_BUFFER_FORMAT_SHORT2,
            Self::Uint8 => sys::_NV_OF_BUFFER_FORMAT_NV_OF_BUFFER_FORMAT_UINT8,
            Self::Uint => sys::_NV_OF_BUFFER_FORMAT_NV_OF_BUFFER_FORMAT_UINT,
        }
    }

    /// Bytes per element in plane 0 for this format.
    pub const fn plane0_elem_bytes(self) -> usize {
        match self {
            Self::Grayscale8 | Self::Nv12 | Self::Uint8 => 1,
            Self::Short => 2,
            Self::Abgr8 | Self::Short2 | Self::Uint => 4,
        }
    }
}

/// Backing CUDA storage type for [`OfBuffer`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CudaBufferType {
    /// `CUdeviceptr` linear pitched GPU buffer.
    #[default]
    CuDevicePtr,
    /// `CUarray` opaque 2D GPU array.
    CuArray,
}

impl CudaBufferType {
    const fn to_sys(self) -> sys::NV_OF_CUDA_BUFFER_TYPE {
        match self {
            Self::CuDevicePtr => sys::_NV_OF_CUDA_BUFFER_TYPE_NV_OF_CUDA_BUFFER_TYPE_CUDEVICEPTR,
            Self::CuArray => sys::_NV_OF_CUDA_BUFFER_TYPE_NV_OF_CUDA_BUFFER_TYPE_CUARRAY,
        }
    }
}

/// Optical flow vector in S10.5 fixed-point format (`NV_OF_FLOW_VECTOR`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct FlowVector {
    /// Horizontal displacement in S10.5 fixed-point (divide by 32.0 for pixels).
    pub flow_x: i16,
    /// Vertical displacement in S10.5 fixed-point (divide by 32.0 for pixels).
    pub flow_y: i16,
}

impl FlowVector {
    /// Converts S10.5 fixed-point components to floating-point pixels `(dx, dy)`.
    pub fn to_pixels(self) -> (f32, f32) {
        (f32::from(self.flow_x) / 32.0, f32::from(self.flow_y) / 32.0)
    }
}

/// Region of Interest rectangle (`NV_OF_ROI_RECT`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct RoiRect {
    /// Start X in pixels (aligned to `32 * out_grid`).
    pub start_x: u32,
    /// Start Y in pixels (aligned to `8 * max(out_grid, 2)`).
    pub start_y: u32,
    /// Width in pixels (`>= 32`, aligned to `32 * out_grid`).
    pub width: u32,
    /// Height in pixels (`>= 16`, aligned to `8 * out_grid`).
    pub height: u32,
}

impl RoiRect {
    /// Validates NVOFA ROI alignment and bounds requirements against `out_grid` and frame size.
    pub fn validate(self, out_grid: GridSize, frame_w: u32, frame_h: u32) -> Result<()> {
        let g = out_grid as u32;
        let align_x = 32 * g;
        let align_sy = 8 * g.max(2);
        let align_h = 8 * g;
        if !self.start_x.is_multiple_of(align_x) || !self.width.is_multiple_of(align_x) {
            return Err(Error::Invalid(
                "ROI start_x and width must align to 32 * out_grid",
            ));
        }
        if !self.start_y.is_multiple_of(align_sy) || !self.height.is_multiple_of(align_h) {
            return Err(Error::Invalid("ROI start_y/height alignment invalid"));
        }
        if self.width < 32 || self.height < 16 || self.width > 8192 || self.height > 8192 {
            return Err(Error::Invalid(
                "ROI dimensions out of range [32..8192] x [16..8192]",
            ));
        }
        let end_x = self
            .start_x
            .checked_add(self.width)
            .ok_or(Error::Invalid("ROI x overflow"))?;
        let end_y = self
            .start_y
            .checked_add(self.height)
            .ok_or(Error::Invalid("ROI y overflow"))?;
        if end_x > frame_w || end_y > frame_h {
            return Err(Error::Invalid("ROI exceeds frame bounds"));
        }
        Ok(())
    }
}

/// Initialization parameters for [`OpticalFlowSession::init`].
#[derive(Clone, Copy, Debug)]
pub struct InitConfig {
    /// Input frame width in pixels.
    pub width: u32,
    /// Input frame height in pixels.
    pub height: u32,
    /// Input buffer pixel format (`Grayscale8`, `Nv12`, or `Abgr8`).
    pub input_format: BufferFormat,
    /// Output flow vector grid size.
    pub out_grid: GridSize,
    /// Optional external hint grid size (`>= out_grid`).
    pub hint_grid: Option<HintGridSize>,
    /// Speed vs quality preset.
    pub perf_level: PerfLevel,
    /// Enable output cost buffer calculation.
    pub enable_output_cost: bool,
    /// Enable ROI-based optical flow estimation.
    pub enable_roi: bool,
}

impl InitConfig {
    /// Default optical-flow session config for `width x height` at `out_grid` with `input_format`.
    pub fn new(width: u32, height: u32, out_grid: GridSize, input_format: BufferFormat) -> Self {
        Self {
            width,
            height,
            input_format,
            out_grid,
            hint_grid: None,
            perf_level: PerfLevel::Slow,
            enable_output_cost: false,
            enable_roi: false,
        }
    }
}

/// Descriptor for allocating an [`OfBuffer`].
#[derive(Clone, Copy, Debug)]
pub struct BufferDesc {
    /// Width in pixels (or grid units for Output/Hint/Cost buffers).
    pub width: u32,
    /// Height in pixels (or grid units for Output/Hint/Cost buffers).
    pub height: u32,
    /// Buffer usage.
    pub usage: BufferUsage,
    /// Buffer pixel/element format.
    pub format: BufferFormat,
}

/// Plane stride metadata returned by [`OfBuffer::stride_info`].
#[derive(Clone, Copy, Debug)]
pub struct StrideInfo {
    /// Number of planes (`1..=3`).
    pub num_planes: u32,
    /// `(stride_x_bytes, stride_y_bytes)` per plane.
    pub planes: [(u32, u32); 3],
}

/// Per-frame execution inputs and outputs for [`OpticalFlowSession::execute`].
pub struct ExecuteParams<'a> {
    /// Current input frame (`BufferUsage::Input`).
    pub input_frame: &'a OfBuffer,
    /// Reference frame (`BufferUsage::Input`).
    pub reference_frame: &'a OfBuffer,
    /// Output flow vector buffer (`BufferUsage::Output`).
    pub output_buffer: &'a mut OfBuffer,
    /// Optional external hint buffer (`BufferUsage::Hint`).
    pub external_hints: Option<&'a OfBuffer>,
    /// Optional output cost buffer (`BufferUsage::Cost`).
    pub output_cost: Option<&'a mut OfBuffer>,
    /// Disable automatic temporal hints from previous `execute` calls.
    pub disable_temporal_hints: bool,
    /// Optional ROIs (requires `InitConfig::enable_roi = true`).
    pub rois: &'a [RoiRect],
}

/// CUDA streams bound by [`OpticalFlowSession::set_io_streams`], retained so the raw handles
/// NVOFA keeps stay valid and so host copies can wait for the work queued on them.
type IoStreams = Option<(Arc<Stream>, Arc<Stream>)>;

/// Checks the buffers of one [`OpticalFlowSession::execute`] call against the session config,
/// so a wrong usage, format or size fails here with a specific message instead of inside the
/// driver with a generic `NV_OF_ERR_INVALID_PARAM`.
fn validate_execute_descs(
    cfg: &InitConfig,
    input: BufferDesc,
    reference: BufferDesc,
    output: BufferDesc,
    hints: Option<BufferDesc>,
    cost: Option<BufferDesc>,
) -> Result<()> {
    let dims = |d: BufferDesc| (d.width, d.height);
    for frame in [input, reference] {
        if frame.usage != BufferUsage::Input
            || frame.format != cfg.input_format
            || dims(frame) != (cfg.width, cfg.height)
        {
            return Err(Error::Invalid(
                "input and reference frames must be Input buffers of the InitConfig size and input_format",
            ));
        }
    }
    let out_grid = cfg.out_grid.grid_dims(cfg.width, cfg.height);
    if output.usage != BufferUsage::Output
        || output.format != BufferFormat::Short2
        || dims(output) != out_grid
    {
        return Err(Error::Invalid(
            "output buffer must be a Short2 Output buffer of out_grid dimensions",
        ));
    }
    match (hints, cfg.hint_grid) {
        (None, _) => {}
        (Some(_), None) => {
            return Err(Error::Invalid(
                "external hints passed but hint_grid was None at init",
            ));
        }
        (Some(h), Some(hint_grid)) => {
            let g = hint_grid as u32;
            if h.usage != BufferUsage::Hint
                || h.format != BufferFormat::Short2
                || dims(h) != (cfg.width.div_ceil(g), cfg.height.div_ceil(g))
            {
                return Err(Error::Invalid(
                    "hint buffer must be a Short2 Hint buffer of hint_grid dimensions",
                ));
            }
        }
    }
    if let Some(c) = cost {
        if !cfg.enable_output_cost {
            return Err(Error::Invalid(
                "output cost buffer passed but enable_output_cost was false at init",
            ));
        }
        if c.usage != BufferUsage::Cost
            || !matches!(c.format, BufferFormat::Uint8 | BufferFormat::Uint)
            || dims(c) != out_grid
        {
            return Err(Error::Invalid(
                "cost buffer must be a Uint8 or Uint Cost buffer of out_grid dimensions",
            ));
        }
    }
    Ok(())
}

/// An NVOFA optical flow session in CUDA mode (`NvOFHandle`).
pub struct OpticalFlowSession {
    raw: sys::NvOFHandle,
    cuda: Arc<Context>,
    list: &'static sys::NV_OF_CUDA_API_FUNCTION_LIST,
    config: Option<InitConfig>,
    /// Maximum ROI count reported by `NV_OF_CAPS_SUPPORT_ROI_MAX_NUM` (0 unless `enable_roi`).
    max_rois: u32,
    /// Serializes every `&self` call into the session handle; also holds the bound I/O streams.
    io: Mutex<IoStreams>,
}

// SAFETY: the session owns its `NvOFHandle`; parent `Context` and retained `Stream`s are `Send`.
unsafe impl Send for OpticalFlowSession {}
// SAFETY: the Optical Flow SDK does not document `NvOFHandle` as thread-safe, so every `&self`
// entry point that passes the handle to NVOFA (`caps`, `set_io_streams`, `execute`) holds the
// `io` mutex for the duration of the native call.
unsafe impl Sync for OpticalFlowSession {}

impl OpticalFlowSession {
    /// Opens an NVOFA CUDA session on `cuda`.
    pub fn open(cuda: &Arc<Context>) -> Result<Arc<Self>> {
        let list = fn_list()?;
        cuda.bind_to_thread()?;
        let create_fn = entry!(list, nvCreateOpticalFlowCuda);
        let mut raw: sys::NvOFHandle = std::ptr::null_mut();
        // SAFETY: `cuda.cu_ctx()` is a live CUDA context and `raw` is a valid out-pointer.
        let status = unsafe { create_fn(cuda.cu_ctx(), &mut raw) };
        check("nvCreateOpticalFlowCuda", status, None, list)?;
        Ok(Arc::new(Self {
            raw,
            cuda: cuda.clone(),
            list,
            config: None,
            max_rois: 0,
            io: Mutex::new(None),
        }))
    }

    /// Queries an NVOFA capability (`nvOFGetCaps`) into `out_vals`, returning the count written.
    pub fn caps(&self, cap: sys::NV_OF_CAPS, out_vals: &mut [u32]) -> Result<u32> {
        let caps_fn = entry!(self.list, nvOFGetCaps);
        let mut size = out_vals.len() as u32;
        let ptr = if out_vals.is_empty() {
            std::ptr::null_mut()
        } else {
            out_vals.as_mut_ptr()
        };
        let _io = self.io.lock().unwrap_or_else(|e| e.into_inner());
        // SAFETY: `self.raw` is live, access is serialized by `io`, and `ptr` has `size` writable
        // `u32` elements.
        let status = unsafe { caps_fn(self.raw, cap, ptr, &mut size) };
        check("nvOFGetCaps", status, Some(self.raw), self.list)?;
        Ok(size)
    }

    /// Initializes the optical flow session with `cfg`.
    pub fn init(self: &mut Arc<Self>, cfg: &InitConfig) -> Result<()> {
        if cfg.width == 0 || cfg.height == 0 {
            return Err(Error::Invalid("frame width and height must be non-zero"));
        }
        if let Some(hg) = cfg.hint_grid
            && (hg as u32) < (cfg.out_grid as u32)
        {
            return Err(Error::Invalid("hint_grid must be >= out_grid"));
        }
        let this = Arc::get_mut(self).ok_or(Error::Invalid(
            "cannot initialize session while buffers exist",
        ))?;
        this.cuda.bind_to_thread()?;
        let init_fn = entry!(this.list, nvOFInit);
        // SAFETY: all-zero bit pattern is valid for `NV_OF_INIT_PARAMS`.
        let mut params: sys::NV_OF_INIT_PARAMS = unsafe { zeroed() };
        params.width = cfg.width;
        params.height = cfg.height;
        params.outGridSize = cfg.out_grid.to_sys();
        params.hintGridSize = cfg.hint_grid.map_or(
            sys::_NV_OF_HINT_VECTOR_GRID_SIZE_NV_OF_HINT_VECTOR_GRID_SIZE_UNDEFINED,
            HintGridSize::to_sys,
        );
        params.mode = sys::_NV_OF_MODE_NV_OF_MODE_OPTICALFLOW;
        params.perfLevel = cfg.perf_level.to_sys();
        params.enableExternalHints = if cfg.hint_grid.is_some() {
            sys::_NV_OF_BOOL_NV_OF_TRUE
        } else {
            sys::_NV_OF_BOOL_NV_OF_FALSE
        };
        params.enableOutputCost = if cfg.enable_output_cost {
            sys::_NV_OF_BOOL_NV_OF_TRUE
        } else {
            sys::_NV_OF_BOOL_NV_OF_FALSE
        };
        params.enableRoi = if cfg.enable_roi {
            sys::_NV_OF_BOOL_NV_OF_TRUE
        } else {
            sys::_NV_OF_BOOL_NV_OF_FALSE
        };
        params.inputBufferFormat = cfg.input_format.to_sys();
        // SAFETY: `this.raw` is live and `params` is populated.
        let status = unsafe { init_fn(this.raw, &params) };
        check("nvOFInit", status, Some(this.raw), this.list)?;
        // The ROI count limit is hardware-reported; read it once so `execute` stays query-free.
        this.max_rois = if cfg.enable_roi {
            let mut max = [0u32; 1];
            this.caps(sys::_NV_OF_CAPS_NV_OF_CAPS_SUPPORT_ROI_MAX_NUM, &mut max)?;
            max[0]
        } else {
            0
        };
        this.config = Some(*cfg);
        Ok(())
    }

    /// Sets the input and output CUDA streams for subsequent [`Self::execute`] calls.
    ///
    /// The session retains both streams: NVOFA keeps their raw handles, and
    /// [`OfBuffer::upload_plane0`] / [`OfBuffer::download_plane0`] wait for the work queued on
    /// them before copying.
    pub fn set_io_streams(&self, input: &Arc<Stream>, output: &Arc<Stream>) -> Result<()> {
        let set_fn = entry!(self.list, nvOFSetIOCudaStreams);
        let mut io = self.io.lock().unwrap_or_else(|e| e.into_inner());
        // SAFETY: `self.raw` is live, access is serialized by `io`, and both CUDA stream handles
        // are valid; they are retained below for as long as the session may use them.
        let status = unsafe { set_fn(self.raw, input.cu_stream(), output.cu_stream()) };
        check("nvOFSetIOCudaStreams", status, Some(self.raw), self.list)?;
        *io = Some((input.clone(), output.clone()));
        Ok(())
    }

    /// Blocks until NVOFA work that may touch session buffers has finished: the bound I/O
    /// streams, or the whole context when none were set (NVOFA then uses the legacy default
    /// stream, which non-blocking caller streams do not order against).
    fn wait_io(&self) -> Result<()> {
        let streams = self.io.lock().unwrap_or_else(|e| e.into_inner()).clone();
        match streams {
            Some((input, output)) => {
                input.synchronize()?;
                output.synchronize()?;
            }
            None => self.cuda.synchronize()?,
        }
        Ok(())
    }

    /// Executes optical flow estimation on `params` with zero heap allocations.
    ///
    /// Buffers are checked against the [`InitConfig`] first: frames must be `Input` buffers of
    /// the configured size and format, the output a `Short2` `Output` buffer of the output grid,
    /// hints and cost buffers must be enabled at init and sized to their grids.
    pub fn execute(&self, params: &mut ExecuteParams<'_>) -> Result<()> {
        let cfg = self
            .config
            .ok_or(Error::Invalid("optical flow session is not initialized"))?;
        if params.input_frame.session.raw != self.raw
            || params.reference_frame.session.raw != self.raw
            || params.output_buffer.session.raw != self.raw
            || params
                .external_hints
                .is_some_and(|b| b.session.raw != self.raw)
            || params
                .output_cost
                .as_ref()
                .is_some_and(|b| b.session.raw != self.raw)
        {
            return Err(Error::Invalid(
                "all OfBuffer handles must belong to this OpticalFlowSession",
            ));
        }
        validate_execute_descs(
            &cfg,
            params.input_frame.desc,
            params.reference_frame.desc,
            params.output_buffer.desc,
            params.external_hints.map(|b| b.desc),
            params.output_cost.as_ref().map(|b| b.desc),
        )?;
        let num_rois = u32::try_from(params.rois.len())
            .map_err(|_| Error::Invalid("ROI count exceeds u32::MAX"))?;
        if num_rois > 0 {
            if !cfg.enable_roi {
                return Err(Error::Invalid(
                    "ROIs passed but enable_roi was false at init",
                ));
            }
            if num_rois > self.max_rois {
                return Err(Error::Unsupported {
                    what: "ROI count",
                    required: num_rois,
                    found: self.max_rois,
                });
            }
            for roi in params.rois {
                roi.validate(cfg.out_grid, cfg.width, cfg.height)?;
            }
        }
        self.cuda.bind_to_thread()?;
        let exec_fn = entry!(self.list, nvOFExecute);
        // SAFETY: all-zero bit pattern is valid for `NV_OF_EXECUTE_INPUT_PARAMS`.
        let mut in_params: sys::NV_OF_EXECUTE_INPUT_PARAMS = unsafe { zeroed() };
        in_params.inputFrame = params.input_frame.raw;
        in_params.referenceFrame = params.reference_frame.raw;
        in_params.externalHints = params
            .external_hints
            .map_or(std::ptr::null_mut(), |b| b.raw);
        in_params.disableTemporalHints = if params.disable_temporal_hints {
            sys::_NV_OF_BOOL_NV_OF_TRUE
        } else {
            sys::_NV_OF_BOOL_NV_OF_FALSE
        };
        in_params.numRois = num_rois;
        in_params.roiData = if params.rois.is_empty() {
            std::ptr::null_mut()
        } else {
            // `RoiRect` is `#[repr(C)]` with identical layout to `NV_OF_ROI_RECT`.
            params
                .rois
                .as_ptr()
                .cast::<sys::NV_OF_ROI_RECT>()
                .cast_mut()
        };

        // SAFETY: all-zero bit pattern is valid for `NV_OF_EXECUTE_OUTPUT_PARAMS`.
        let mut out_params: sys::NV_OF_EXECUTE_OUTPUT_PARAMS = unsafe { zeroed() };
        out_params.outputBuffer = params.output_buffer.raw;
        out_params.outputCostBuffer = params
            .output_cost
            .as_ref()
            .map_or(std::ptr::null_mut(), |b| b.raw);

        let _io = self.io.lock().unwrap_or_else(|e| e.into_inner());
        // SAFETY: `self.raw` is initialized, access is serialized by `io`, and all buffer handles
        // belong to this session and match its `InitConfig` (validated above).
        let status = unsafe { exec_fn(self.raw, &in_params, &mut out_params) };
        check("nvOFExecute", status, Some(self.raw), self.list)
    }

    /// Returns the initialization config if initialized.
    pub const fn config(&self) -> Option<InitConfig> {
        self.config
    }
}

impl Drop for OpticalFlowSession {
    fn drop(&mut self) {
        if self.raw.is_null() {
            return;
        }
        if !teardown::bind(&self.cuda) {
            return;
        }
        if let Some(destroy_fn) = self.list.nvOFDestroy {
            // SAFETY: `self.raw` is owned by this session and all child `OfBuffer`s have dropped.
            let status = unsafe { destroy_fn(self.raw) };
            teardown::check(
                Library::Nvofa,
                "nvOFDestroy",
                i64::from(status),
                i64::from(sys::_NV_OF_STATUS_NV_OF_SUCCESS),
            );
        }
    }
}

/// GPU buffer managed by NVOFA (`NvOFGPUBufferHandle`).
pub struct OfBuffer {
    raw: sys::NvOFGPUBufferHandle,
    session: Arc<OpticalFlowSession>,
    desc: BufferDesc,
    kind: CudaBufferType,
}

impl OfBuffer {
    /// Allocates an NVOFA GPU buffer on `session`.
    pub fn new(
        session: &Arc<OpticalFlowSession>,
        desc: BufferDesc,
        kind: CudaBufferType,
    ) -> Result<Self> {
        if desc.width == 0 || desc.height == 0 {
            return Err(Error::Invalid("buffer dimensions must be non-zero"));
        }
        session.cuda.bind_to_thread()?;
        let create_fn = entry!(session.list, nvOFCreateGPUBufferCuda);
        let raw_desc = sys::NV_OF_BUFFER_DESCRIPTOR {
            width: desc.width,
            height: desc.height,
            bufferUsage: desc.usage.to_sys(),
            bufferFormat: desc.format.to_sys(),
        };
        let mut raw: sys::NvOFGPUBufferHandle = std::ptr::null_mut();
        // SAFETY: `session.raw` is live and `raw_desc`/`raw` are valid pointers.
        let status = unsafe { create_fn(session.raw, &raw_desc, kind.to_sys(), &mut raw) };
        check(
            "nvOFCreateGPUBufferCuda",
            status,
            Some(session.raw),
            session.list,
        )?;
        Ok(Self {
            raw,
            session: session.clone(),
            desc,
            kind,
        })
    }

    /// Queries the hardware plane strides of this buffer.
    pub fn stride_info(&self) -> Result<StrideInfo> {
        let stride_fn = entry!(self.session.list, nvOFGPUBufferGetStrideInfo);
        // SAFETY: all-zero bit pattern is valid for `NV_OF_CUDA_BUFFER_STRIDE_INFO`.
        let mut info: sys::NV_OF_CUDA_BUFFER_STRIDE_INFO = unsafe { zeroed() };
        // SAFETY: `self.raw` is a live buffer handle.
        let status = unsafe { stride_fn(self.raw, &mut info) };
        check(
            "nvOFGPUBufferGetStrideInfo",
            status,
            Some(self.session.raw),
            self.session.list,
        )?;
        Ok(StrideInfo {
            num_planes: info.numPlanes,
            planes: [
                (
                    info.strideInfo[0].strideXInBytes,
                    info.strideInfo[0].strideYInBytes,
                ),
                (
                    info.strideInfo[1].strideXInBytes,
                    info.strideInfo[1].strideYInBytes,
                ),
                (
                    info.strideInfo[2].strideXInBytes,
                    info.strideInfo[2].strideYInBytes,
                ),
            ],
        })
    }

    /// Returns the underlying `CUdeviceptr` if created with [`CudaBufferType::CuDevicePtr`].
    pub fn cu_deviceptr(&self) -> Option<cuda_core::sys::CUdeviceptr> {
        if self.kind != CudaBufferType::CuDevicePtr {
            return None;
        }
        let get_fn = self.session.list.nvOFGPUBufferGetCUdeviceptr?;
        // SAFETY: `self.raw` is a live `CuDevicePtr` buffer.
        let ptr = unsafe { get_fn(self.raw) };
        if ptr == 0 { None } else { Some(ptr) }
    }

    /// Returns the underlying `CUarray` if created with [`CudaBufferType::CuArray`].
    pub fn cu_array(&self) -> Option<cuda_core::sys::CUarray> {
        if self.kind != CudaBufferType::CuArray {
            return None;
        }
        let get_fn = self.session.list.nvOFGPUBufferGetCUarray?;
        // SAFETY: `self.raw` is a live `CuArray` buffer.
        let arr = unsafe { get_fn(self.raw) };
        if arr.is_null() { None } else { Some(arr) }
    }

    fn validate_plane0_bounds(
        &self,
        width_bytes: usize,
        height: usize,
        slice_len: usize,
    ) -> Result<usize> {
        if width_bytes == 0 || height == 0 {
            return Err(Error::Invalid(
                "plane0 copy width_bytes and height must be non-zero",
            ));
        }
        if height > self.desc.height as usize {
            return Err(Error::Invalid(
                "plane0 copy height exceeds buffer descriptor height",
            ));
        }
        let max_row_bytes = (self.desc.width as usize)
            .checked_mul(self.desc.format.plane0_elem_bytes())
            .ok_or(Error::Invalid("buffer row byte size overflow"))?;
        if width_bytes > max_row_bytes {
            return Err(Error::Invalid(
                "plane0 copy width_bytes exceeds buffer descriptor width",
            ));
        }
        let expected = width_bytes
            .checked_mul(height)
            .ok_or(Error::Invalid("plane0 copy size overflow"))?;
        if slice_len != expected {
            return Err(Error::Invalid(
                "slice length does not match width_bytes * height",
            ));
        }
        let stride = self.stride_info()?.planes[0].0 as usize;
        if width_bytes > stride {
            return Err(Error::Invalid(
                "plane0 copy width_bytes exceeds GPU buffer stride",
            ));
        }
        Ok(stride)
    }

    /// Uploads tightly packed host bytes into plane 0 of a `CuDevicePtr` buffer on `stream`.
    pub fn upload_plane0(
        &self,
        stream: &Stream,
        width_bytes: usize,
        height: usize,
        src: &[u8],
    ) -> Result<()> {
        let dev_ptr = self
            .cu_deviceptr()
            .ok_or(Error::Invalid("upload_plane0 requires CuDevicePtr buffer"))?;
        let stride = self.validate_plane0_bounds(width_bytes, height, src.len())?;
        // A previous `execute` on the session's input stream may still read this buffer.
        self.session.wait_io()?;
        self.session.cuda.bind_to_thread()?;
        // SAFETY: `src` and `dev_ptr` cover `height` rows of `width_bytes` (`<= stride`); the copy
        // waits for `stream`.
        unsafe {
            media::copy_2d_host_to_device(
                stream,
                src.as_ptr().cast(),
                dev_ptr,
                stride,
                width_bytes,
                height,
            )
        }?;
        Ok(())
    }

    /// Downloads plane 0 of a `CuDevicePtr` buffer into tightly packed `dst` on `stream`.
    pub fn download_plane0(
        &self,
        stream: &Stream,
        width_bytes: usize,
        height: usize,
        dst: &mut [u8],
    ) -> Result<()> {
        let dev_ptr = self.cu_deviceptr().ok_or(Error::Invalid(
            "download_plane0 requires CuDevicePtr buffer",
        ))?;
        let stride = self.validate_plane0_bounds(width_bytes, height, dst.len())?;
        // `execute` writes this buffer on the session's output stream, which `stream` is not
        // ordered against.
        self.session.wait_io()?;
        self.session.cuda.bind_to_thread()?;
        // SAFETY: `dev_ptr` and `dst` cover `height` rows of `width_bytes` (`<= stride`); the copy
        // waits for `stream`.
        unsafe {
            media::copy_2d_device_to_host(
                stream,
                dev_ptr,
                stride,
                dst.as_mut_ptr().cast(),
                width_bytes,
                height,
            )
        }?;
        Ok(())
    }

    /// Downloads [`FlowVector`]s from a `Short2` output buffer into `dst` with zero allocations.
    pub fn download_flow_vectors(&self, stream: &Stream, dst: &mut [FlowVector]) -> Result<()> {
        if self.desc.format != BufferFormat::Short2 {
            return Err(Error::Invalid(
                "download_flow_vectors requires Short2 buffer format",
            ));
        }
        let count = (self.desc.width as usize)
            .checked_mul(self.desc.height as usize)
            .ok_or(Error::Invalid("flow vector count overflow"))?;
        if dst.len() != count {
            return Err(Error::Invalid(
                "destination slice length does not match grid_w * grid_h",
            ));
        }
        let width_bytes = (self.desc.width as usize) * size_of::<FlowVector>();
        let byte_len = std::mem::size_of_val(dst);
        // SAFETY: `FlowVector` is `#[repr(C)]` POD (`i16`, `i16`), valid to view as mutable bytes.
        let dst_bytes =
            unsafe { std::slice::from_raw_parts_mut(dst.as_mut_ptr().cast::<u8>(), byte_len) };
        self.download_plane0(stream, width_bytes, self.desc.height as usize, dst_bytes)
    }

    /// Buffer descriptor.
    pub const fn desc(&self) -> BufferDesc {
        self.desc
    }
}

impl Drop for OfBuffer {
    fn drop(&mut self) {
        // Bind the context as the session's `Drop` does: the buffer owns CUDA memory and may drop
        // on a thread with no current context. A bind failure is recorded; the destroy still
        // runs so that one sticky error does not leak every remaining buffer.
        if let Err(e) = self.session.cuda.bind_to_thread() {
            self.session.cuda.record_err(Err::<(), _>(e));
        }
        if let Some(destroy_fn) = self.session.list.nvOFDestroyGPUBufferCuda {
            // SAFETY: `self.raw` was created on `self.session`.
            let status = unsafe { destroy_fn(self.raw) };
            teardown::check(
                Library::Nvofa,
                "nvOFDestroyGPUBufferCuda",
                i64::from(status),
                i64::from(sys::_NV_OF_STATUS_NV_OF_SUCCESS),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_dims_ceil_division() {
        assert_eq!(GridSize::Four.grid_dims(1920, 1080), (480, 270));
        assert_eq!(GridSize::Four.grid_dims(129, 65), (33, 17));
        assert_eq!(GridSize::One.grid_dims(128, 128), (128, 128));
    }

    #[test]
    fn flow_vector_s10_5_conversion() {
        let fv = FlowVector {
            flow_x: 32 * 8,
            flow_y: -32 * 4,
        };
        assert_eq!(fv.to_pixels(), (8.0, -4.0));
        assert_eq!(size_of::<FlowVector>(), size_of::<sys::NV_OF_FLOW_VECTOR>());
    }

    #[test]
    fn roi_rect_validation_enforces_alignment_and_bounds() {
        let valid = RoiRect {
            start_x: 0,
            start_y: 0,
            width: 128,
            height: 32,
        };
        assert!(valid.validate(GridSize::Four, 256, 256).is_ok());

        let bad_x = RoiRect {
            start_x: 16,
            start_y: 0,
            width: 128,
            height: 32,
        };
        assert!(bad_x.validate(GridSize::Four, 256, 256).is_err());

        let out_of_bounds = RoiRect {
            start_x: 128,
            start_y: 0,
            width: 256,
            height: 32,
        };
        assert!(out_of_bounds.validate(GridSize::Four, 256, 256).is_err());

        assert_eq!(size_of::<RoiRect>(), size_of::<sys::NV_OF_ROI_RECT>());
    }

    /// OFA-3: execute buffers are checked against usage, format and grid size before NVOFA.
    #[test]
    fn execute_desc_validation_rejects_mismatched_buffers() {
        let mut cfg = InitConfig::new(129, 65, GridSize::Four, BufferFormat::Grayscale8);
        let desc = |width, height, usage, format| BufferDesc {
            width,
            height,
            usage,
            format,
        };
        let frame = desc(129, 65, BufferUsage::Input, BufferFormat::Grayscale8);
        let out = desc(33, 17, BufferUsage::Output, BufferFormat::Short2);
        let cost = desc(33, 17, BufferUsage::Cost, BufferFormat::Uint8);
        assert!(validate_execute_descs(&cfg, frame, frame, out, None, None).is_ok());

        let wrong_format = desc(129, 65, BufferUsage::Input, BufferFormat::Nv12);
        assert!(validate_execute_descs(&cfg, wrong_format, frame, out, None, None).is_err());
        let wrong_size = desc(128, 65, BufferUsage::Input, BufferFormat::Grayscale8);
        assert!(validate_execute_descs(&cfg, frame, wrong_size, out, None, None).is_err());
        assert!(validate_execute_descs(&cfg, frame, frame, cost, None, None).is_err());
        let small_out = desc(32, 17, BufferUsage::Output, BufferFormat::Short2);
        assert!(validate_execute_descs(&cfg, frame, frame, small_out, None, None).is_err());

        // Cost needs `enable_output_cost`; hints need `hint_grid` and its grid size.
        assert!(validate_execute_descs(&cfg, frame, frame, out, None, Some(cost)).is_err());
        cfg.enable_output_cost = true;
        assert!(validate_execute_descs(&cfg, frame, frame, out, None, Some(cost)).is_ok());
        assert!(validate_execute_descs(&cfg, frame, frame, out, None, Some(out)).is_err());

        let hint = desc(17, 9, BufferUsage::Hint, BufferFormat::Short2);
        assert!(validate_execute_descs(&cfg, frame, frame, out, Some(hint), None).is_err());
        cfg.hint_grid = Some(HintGridSize::Eight);
        assert!(validate_execute_descs(&cfg, frame, frame, out, Some(hint), None).is_ok());
        let wrong_hint = desc(33, 17, BufferUsage::Hint, BufferFormat::Short2);
        assert!(validate_execute_descs(&cfg, frame, frame, out, Some(wrong_hint), None).is_err());
    }

    #[test]
    fn status_code_is_copy_and_displays() {
        let code = StatusCode(sys::_NV_OF_STATUS_NV_OF_ERR_INVALID_PARAM);
        let copy = code;
        assert_eq!(code, copy);
        assert!(code.to_string().contains("NVOFA status"));
    }
}
