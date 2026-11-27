//! CUDA GPU column-span baker for `csg-scene` R0 [`csg_scene::voxel_volume::VoxelVolume`].
//!
//! **What.** Implements [`GpuVoxelBaker`], which compiles and disk-caches `csg-scene`'s
//! `shaders/voxel_bake.cuh` (`voxel_bake_trace_columns` and `voxel_bake_merge_columns`) via NVRTC
//! and executes the two-pass analytical CSG column-span bake on a dedicated CUDA stream before
//! feeding the unpacked spans into [`csg_scene::voxel_volume::VoxelVolume::from_baked_columns`].
//! **Why.** Baking a full city scene (such as Gundo) on CPU takes ~2 s, stalling Flight Lab
//! auto-planning on cache misses. Offloading the per-column vertical CSG ray trace and exact 1D
//! event-sweep merge to CUDA cuts the bake to ~265 ms while checking GPU overflow counters so any
//! capacity overflow falls back cleanly to exact CPU `VoxelVolume::build` without silent
//! truncation.
//! **How.** [`GpuVoxelBaker::new`] acquires `CudaContext::new(0)`, creates a dedicated CUDA
//! [`cuda_core::Stream`], loads the NVRTC-compiled `CUmodule` through
//! [`crate::gpu_export::load_ptx_cached`] (< 5 ms warm startup), and resolves both kernel
//! entry points via [`crate::frame_plan_runner::load_cuda_fn`].
//! [`GpuVoxelBaker::bake_voxel_volume`] flattens `world` via
//! [`csg_scene::voxel_volume::VoxelBakeInput::from_world`], uploads all input tables (allocating
//! a 1-element zeroed dummy buffer whenever a table is empty so every device pointer is non-null),
//! zeroes only the count/overflow buffers with `cuMemsetD8Async`, launches both passes on
//! `self.stream`, synchronizes once, rejects any non-zero overflow counter, and downloads and
//! unpacks the merged spans into a complete [`csg_scene::voxel_volume::VoxelVolume`].
//! **Uses.** `bytemuck`, `csg_device`, `csg_scene`, `cuda_bindings`, `cuda_core`,
//! `crate::frame_plan_runner::load_cuda_fn`, `crate::gpu_export::load_ptx_cached`.
//! **Used by.** `crate::flight::oracle`.

use std::ffi::c_void;
use std::sync::Arc;

use bytemuck::Pod;
use csg_scene::WorldScene;
use csg_scene::voxel_volume::{
    ColumnSpanPod, VoxelBakeInput, VoxelVolume, VoxelVolumeConfig, voxel_bake_cuh_source,
};
use cuda_bindings as cu;
use cuda_core::{Context as CudaContext, DeviceBuffer, Stream};

use crate::frame_plan_runner::load_cuda_fn;
use crate::gpu_export::load_ptx_cached;

/// CUDA threads per block for both linear 1-thread-per-column voxel bake kernels.
const BLOCK_SIZE: u32 = 128;

/// Target NVRTC virtual architecture shared across `nv-demo` compute kernels.
const NVRTC_ARCH: &str = "compute_75";

/// Persistent grow-only VRAM buffer for [`GpuVoxelBaker`], eliminating per-bake `cuMemAllocAsync`
/// / `cuMemFreeAsync` churn across repeated Flight Lab voxel bakes.
#[derive(Default)]
struct GrowVoxelBuf {
    buf: Option<DeviceBuffer<u8>>,
    cap_bytes: usize,
}

impl GrowVoxelBuf {
    /// Ensures at least `bytes` (`>= 1`) of device memory are allocated (growing in power-of-two
    /// steps) and returns the non-null device pointer.
    fn ensure_uninit(
        &mut self,
        stream: &Arc<Stream>,
        label: &str,
        bytes: usize,
    ) -> Result<u64, String> {
        if bytes == 0 {
            return Err(format!(
                "GPU voxel bake '{label}' allocation size must be > 0"
            ));
        }
        if bytes > self.cap_bytes || self.buf.is_none() {
            let alloc_bytes = bytes.max(256).next_power_of_two();
            // SAFETY: every byte read by kernels or host readbacks from this buffer is written
            // first on `stream` via `cuMemcpyHtoDAsync_v2`, `cuMemsetD8Async`, or kernel outputs,
            // and `bake_voxel_volume` / `Drop` synchronize `stream` before reallocation or drop.
            let new_buf = unsafe { DeviceBuffer::<u8>::uninitialized_async(stream, alloc_bytes) }
                .map_err(|e| format!("CUDA alloc '{label}' ({alloc_bytes} B) failed: {e}"))?;
            self.buf = Some(new_buf);
            self.cap_bytes = alloc_bytes;
        }
        self.buf
            .as_ref()
            .map(DeviceBuffer::cu_deviceptr)
            .ok_or_else(|| format!("GPU voxel bake '{label}' device buffer missing"))
    }

    /// Uploads `slice` asynchronously into this persistent buffer (or zeroes a 1-element dummy
    /// region when `slice` is empty so every kernel pointer remains non-null).
    fn upload_or_dummy<T: Pod>(
        &mut self,
        stream: &Arc<Stream>,
        label: &str,
        slice: &[T],
    ) -> Result<u64, String> {
        if slice.is_empty() {
            let dummy_bytes = std::mem::size_of::<T>().max(1);
            let ptr = self.ensure_uninit(stream, label, dummy_bytes)?;
            self.memset_zero_async(stream, label, dummy_bytes)?;
            return Ok(ptr);
        }
        let bytes: &[u8] = bytemuck::cast_slice(slice);
        let ptr = self.ensure_uninit(stream, label, bytes.len())?;
        // SAFETY: `ptr` owns at least `bytes.len()` 256-byte-aligned device bytes on `stream`,
        // and `bytes` points to valid initialized `Pod` host memory live across the copy.
        let rc = unsafe {
            cu::cuMemcpyHtoDAsync_v2(ptr, bytes.as_ptr().cast(), bytes.len(), stream.cu_stream())
        };
        if rc != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!("cuMemcpyHtoDAsync_v2('{label}') failed ({rc:?})"));
        }
        Ok(ptr)
    }

    /// Asynchronously zeroes the first `bytes` of this persistent buffer on `stream`.
    fn memset_zero_async(
        &self,
        stream: &Stream,
        label: &str,
        bytes: usize,
    ) -> Result<(), String> {
        if bytes > self.cap_bytes {
            return Err(format!(
                "GPU voxel bake '{label}' memset size {bytes} B exceeds capacity {} B",
                self.cap_bytes
            ));
        }
        let ptr = self
            .buf
            .as_ref()
            .map(DeviceBuffer::cu_deviceptr)
            .ok_or_else(|| format!("GPU voxel bake '{label}' buffer not allocated"))?;
        // SAFETY: `ptr` owns at least `bytes` valid device bytes on `stream`.
        let rc = unsafe { cu::cuMemsetD8Async(ptr, 0u8, bytes, stream.cu_stream()) };
        if rc != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!("cuMemsetD8Async('{label}') failed ({rc:?})"));
        }
        Ok(())
    }

    /// Downloads a single `u32` after `stream` has been synchronized.
    fn download_u32(&self, label: &str) -> Result<u32, String> {
        let size = std::mem::size_of::<u32>();
        if size > self.cap_bytes {
            return Err(format!(
                "GPU voxel bake '{label}' download exceeds capacity {} B",
                self.cap_bytes
            ));
        }
        let ptr = self
            .buf
            .as_ref()
            .map(DeviceBuffer::cu_deviceptr)
            .ok_or_else(|| format!("GPU voxel bake '{label}' buffer not allocated"))?;
        let mut val = 0u32;
        // SAFETY: caller synchronized `stream`, `ptr` owns >= 4 device bytes, and `val` is writable.
        let rc = unsafe { cu::cuMemcpyDtoH_v2((&raw mut val).cast(), ptr, size) };
        if rc != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!("cuMemcpyDtoH_v2('{label}') failed ({rc:?})"));
        }
        Ok(val)
    }

    /// Downloads `count` elements of `T: Pod` into reusable host scratch `out` after `stream` sync.
    fn download_into<T: Pod>(
        &self,
        label: &str,
        count: usize,
        out: &mut Vec<T>,
    ) -> Result<(), String> {
        let bytes = checked_bytes::<T>(count, label)?;
        out.resize(count, T::zeroed());
        if bytes == 0 {
            return Ok(());
        }
        if bytes > self.cap_bytes {
            return Err(format!(
                "GPU voxel bake '{label}' download size {bytes} B exceeds capacity {} B",
                self.cap_bytes
            ));
        }
        let ptr = self
            .buf
            .as_ref()
            .map(DeviceBuffer::cu_deviceptr)
            .ok_or_else(|| format!("GPU voxel bake '{label}' buffer not allocated"))?;
        let raw: &mut [u8] = bytemuck::cast_slice_mut(out.as_mut_slice());
        // SAFETY: caller synchronized `stream`, `ptr` owns >= `raw.len()` device bytes, and `raw`
        // points to `raw.len()` writable `Pod` host bytes.
        let rc = unsafe { cu::cuMemcpyDtoH_v2(raw.as_mut_ptr().cast(), ptr, raw.len()) };
        if rc != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!("cuMemcpyDtoH_v2('{label}') failed ({rc:?})"));
        }
        Ok(())
    }
}

/// All 18 persistent grow-only VRAM buffers and reusable host readback scratch vectors owned by
/// [`GpuVoxelBaker`], eliminating per-bake GPU allocation/free churn once warm.
#[derive(Default)]
struct VoxelBakeBuffers {
    grid: GrowVoxelBuf,
    items: GrowVoxelBuf,
    unbounded: GrowVoxelBuf,
    tile_starts: GrowVoxelBuf,
    tile_items: GrowVoxelBuf,
    compounds: GrowVoxelBuf,
    ops: GrowVoxelBuf,
    prims: GrowVoxelBuf,
    aux: GrowVoxelBuf,
    guards: GrowVoxelBuf,
    repeats: GrowVoxelBuf,
    raw_spans: GrowVoxelBuf,
    raw_counts: GrowVoxelBuf,
    coords: GrowVoxelBuf,
    out_spans: GrowVoxelBuf,
    out_counts: GrowVoxelBuf,
    raw_overflow: GrowVoxelBuf,
    out_overflow: GrowVoxelBuf,
    host_out_spans: Vec<ColumnSpanPod>,
    host_out_counts: Vec<u32>,
}

/// Reusable CUDA driver baker executing `csg-scene`'s two-pass `voxel_bake.cuh` kernels.
///
/// Why: holding the primary CUDA context, a dedicated stream, the disk-cached `CUmodule`, and all
/// 18 grow-only VRAM buffers across Auto-Plan jobs amortizes NVRTC compilation to < 5 ms and
/// eliminates steady-state VRAM alloc/free churn while keeping the background
/// `"nv-flight-planner"` thread isolated from the OptiX render stream.
pub struct GpuVoxelBaker {
    // Dropped before `stream` and `cuda` (and after `Drop::drop` synchronizes `self.stream`).
    buffers: VoxelBakeBuffers,
    cuda: Arc<CudaContext>,
    stream: Arc<Stream>,
    module: cu::CUmodule,
    trace_fn: cu::CUfunction,
    merge_fn: cu::CUfunction,
}

// SAFETY: `CUmodule` and `CUfunction` belong to `self.cuda`, `Arc<CudaContext>` and `Arc<Stream>`
// are `Send`, every method binds `self.cuda` to the calling thread before invoking CUDA driver
// APIs, and `Drop` binds `self.cuda` before unloading `self.module`.
unsafe impl Send for GpuVoxelBaker {}

impl GpuVoxelBaker {
    /// Initializes CUDA context 0, creates a dedicated stream, compiles or loads the cached
    /// `voxel_bake.cuh` PTX module, and resolves `voxel_bake_trace_columns` and
    /// `voxel_bake_merge_columns`.
    ///
    /// Why: fails with a descriptive `String` on machines without a CUDA driver, GPU, or NVRTC
    /// runtime so [`crate::flight::oracle`] can fall back transparently to CPU
    /// [`VoxelVolume::build`].
    ///
    /// # Errors
    /// Returns an error if CUDA context/stream initialization, NVRTC compilation, PTX module
    /// loading, or kernel symbol lookup fails.
    pub fn new() -> Result<Self, String> {
        let cuda = CudaContext::new(0)
            .map_err(|e| format!("CUDA context 0 initialization failed: {e}"))?;
        cuda.bind_to_thread()
            .map_err(|e| format!("CUDA bind_to_thread failed: {e}"))?;
        let stream = cuda
            .new_stream()
            .map_err(|e| format!("CUDA voxel bake stream creation failed: {e}"))?;

        let src = voxel_bake_cuh_source(csg_device::CSG_EVAL_CUH);
        let module = load_ptx_cached(&src, "voxel_bake.cu", NVRTC_ARCH, &[], &[], |ptx| {
            let mut m: cu::CUmodule = std::ptr::null_mut();
            // SAFETY: `ptx` is a NUL-terminated PTX image produced by NVRTC or the verified disk
            // cache, and `cuda` is bound to the current thread.
            let rc = unsafe { cu::cuModuleLoadData(&mut m, ptx.as_ptr().cast()) };
            if rc != cu::cudaError_enum_CUDA_SUCCESS || m.is_null() {
                return Err(format!("cuModuleLoadData(voxel_bake.cu) failed ({rc:?})"));
            }
            Ok(m)
        })?;

        let trace_fn = match load_cuda_fn(module, "voxel_bake_trace_columns") {
            Ok(f) => f,
            Err(err) => {
                unload_module_best_effort(module, "init error");
                return Err(err);
            }
        };
        let merge_fn = match load_cuda_fn(module, "voxel_bake_merge_columns") {
            Ok(f) => f,
            Err(err) => {
                unload_module_best_effort(module, "init error");
                return Err(err);
            }
        };

        Ok(Self {
            buffers: VoxelBakeBuffers::default(),
            cuda,
            stream,
            module,
            trace_fn,
            merge_fn,
        })
    }

    /// Bakes a complete [`VoxelVolume`] for `world` using CUDA for the per-column CSG ray trace
    /// and deterministic 1D event-sweep merge, then builds the sparse ESDF bricks via
    /// [`VoxelVolume::from_baked_columns`].
    ///
    /// Why: keeps column-span semantics bit-identical to CPU `VoxelVolume::build` while detecting
    /// any GPU raw-span, retrace, or merged-span overflow explicitly via device counters so callers
    /// fall back to exact CPU baking rather than accepting truncated geometry.
    ///
    /// # Errors
    /// Returns an error if `world` produces an empty grid, CUDA allocation/transfer/launch fails,
    /// any GPU span overflow counter is non-zero, or column unpacking/volume assembly fails.
    pub fn bake_voxel_volume(
        &mut self,
        world: &WorldScene,
        cfg: &VoxelVolumeConfig,
    ) -> Result<VoxelVolume, String> {
        self.cuda
            .bind_to_thread()
            .map_err(|e| format!("CUDA bind_to_thread failed before voxel bake: {e}"))?;

        let input = VoxelBakeInput::from_world(world, cfg);
        let nx = input.grid.col_nx as usize;
        let nz = input.grid.col_nz as usize;
        let total_cols = nx
            .checked_mul(nz)
            .ok_or_else(|| format!("GPU voxel bake column count overflow ({nx} x {nz})"))?;
        if total_cols == 0 || !input.bounds.is_finite() || input.bounds.is_empty() {
            return Err("GPU voxel bake received empty world bounds or zero columns".to_string());
        }
        let total_cols_u32 = u32::try_from(total_cols)
            .map_err(|_| format!("GPU voxel bake column count {total_cols} overflows u32"))?;

        let raw_cap = input.grid.raw_cap as usize;
        let out_cap = input.grid.out_cap as usize;
        if raw_cap == 0 || out_cap == 0 {
            return Err(format!(
                "GPU voxel bake received zero span capacity (raw_cap={raw_cap}, out_cap={out_cap})"
            ));
        }

        let lib = &world.library;
        let mut compound_count = u32::try_from(lib.compounds.len()).map_err(|_| {
            format!(
                "GPU voxel bake compound count {} overflows u32",
                lib.compounds.len()
            )
        })?;

        let grid_slice = [input.grid];
        let mut p_grid = self
            .buffers
            .grid
            .upload_or_dummy(&self.stream, "grid", &grid_slice)?;
        let mut p_items = self
            .buffers
            .items
            .upload_or_dummy(&self.stream, "items", &input.items)?;
        let mut p_unbounded = self.buffers.unbounded.upload_or_dummy(
            &self.stream,
            "unbounded_items",
            &input.unbounded_items,
        )?;
        let mut p_tile_starts = self.buffers.tile_starts.upload_or_dummy(
            &self.stream,
            "tile_starts",
            &input.tile_starts,
        )?;
        let mut p_tile_items = self.buffers.tile_items.upload_or_dummy(
            &self.stream,
            "tile_items",
            &input.tile_items,
        )?;

        let mut p_compounds =
            self.buffers
                .compounds
                .upload_or_dummy(&self.stream, "compounds", &lib.compounds)?;
        let mut p_ops = self
            .buffers
            .ops
            .upload_or_dummy(&self.stream, "ops", &lib.ops)?;
        let mut p_prims = self
            .buffers
            .prims
            .upload_or_dummy(&self.stream, "prims", &lib.prims)?;
        let mut p_aux = self
            .buffers
            .aux
            .upload_or_dummy(&self.stream, "aux_planes", &lib.aux_planes)?;
        let mut p_guards =
            self.buffers
                .guards
                .upload_or_dummy(&self.stream, "guard_aabbs", &lib.guard_aabbs)?;
        let mut p_repeats =
            self.buffers
                .repeats
                .upload_or_dummy(&self.stream, "repeats", &lib.repeats)?;

        let raw_spans_elems = total_cols
            .checked_mul(raw_cap)
            .ok_or_else(|| "GPU voxel bake raw_spans element count overflow".to_string())?;
        let coord_elems = raw_spans_elems
            .checked_mul(2)
            .ok_or_else(|| "GPU voxel bake coord_buf element count overflow".to_string())?;
        let out_spans_elems = total_cols
            .checked_mul(out_cap)
            .ok_or_else(|| "GPU voxel bake out_spans element count overflow".to_string())?;

        let raw_spans_bytes = checked_bytes::<ColumnSpanPod>(raw_spans_elems, "raw_spans")?;
        let raw_counts_bytes = checked_bytes::<u32>(total_cols, "raw_counts")?;
        let coord_bytes = checked_bytes::<f32>(coord_elems, "coord_buf")?;
        let out_spans_bytes = checked_bytes::<ColumnSpanPod>(out_spans_elems, "out_spans")?;
        let out_counts_bytes = checked_bytes::<u32>(total_cols, "out_counts")?;
        let u32_bytes = std::mem::size_of::<u32>();

        let mut p_raw_spans =
            self.buffers
                .raw_spans
                .ensure_uninit(&self.stream, "raw_spans", raw_spans_bytes)?;
        let mut p_raw_counts =
            self.buffers
                .raw_counts
                .ensure_uninit(&self.stream, "raw_counts", raw_counts_bytes)?;
        let mut p_coords =
            self.buffers
                .coords
                .ensure_uninit(&self.stream, "coord_buf", coord_bytes)?;
        let mut p_out_spans =
            self.buffers
                .out_spans
                .ensure_uninit(&self.stream, "out_spans", out_spans_bytes)?;
        let mut p_out_counts =
            self.buffers
                .out_counts
                .ensure_uninit(&self.stream, "out_counts", out_counts_bytes)?;
        let mut p_raw_overflow = self.buffers.raw_overflow.ensure_uninit(
            &self.stream,
            "overflow_count",
            u32_bytes,
        )?;
        let mut p_out_overflow = self.buffers.out_overflow.ensure_uninit(
            &self.stream,
            "out_overflow_count",
            u32_bytes,
        )?;

        self.buffers
            .raw_counts
            .memset_zero_async(&self.stream, "raw_counts", raw_counts_bytes)?;
        self.buffers
            .out_counts
            .memset_zero_async(&self.stream, "out_counts", out_counts_bytes)?;
        self.buffers
            .raw_overflow
            .memset_zero_async(&self.stream, "overflow_count", u32_bytes)?;
        self.buffers.out_overflow.memset_zero_async(
            &self.stream,
            "out_overflow_count",
            u32_bytes,
        )?;

        let blocks = total_cols_u32.div_ceil(BLOCK_SIZE);

        let mut trace_args: [*mut c_void; 15] = [
            (&raw mut p_grid).cast(),
            (&raw mut p_items).cast(),
            (&raw mut p_unbounded).cast(),
            (&raw mut p_tile_starts).cast(),
            (&raw mut p_tile_items).cast(),
            (&raw mut p_compounds).cast(),
            (&raw mut p_ops).cast(),
            (&raw mut p_prims).cast(),
            (&raw mut p_aux).cast(),
            (&raw mut p_guards).cast(),
            (&raw mut p_repeats).cast(),
            (&raw mut compound_count).cast(),
            (&raw mut p_raw_spans).cast(),
            (&raw mut p_raw_counts).cast(),
            (&raw mut p_raw_overflow).cast(),
        ];

        // SAFETY: `self.trace_fn` is `voxel_bake_trace_columns` from `voxel_bake.cuh`, which
        // accepts these exact 15 parameters (11 read-only device pointers, `uint32_t
        // compound_count`, and 3 writable device pointers). Every pointer targets a live,
        // non-null, appropriately sized device allocation on `self.stream`, `trace_args` points
        // into live stack variables for the call duration, and `self.stream` is synchronized
        // before any buffer drops.
        let rc_trace = unsafe {
            cu::cuLaunchKernel(
                self.trace_fn,
                blocks,
                1,
                1,
                BLOCK_SIZE,
                1,
                1,
                0,
                self.stream.cu_stream(),
                trace_args.as_mut_ptr(),
                std::ptr::null_mut(),
            )
        };
        if rc_trace != cu::cudaError_enum_CUDA_SUCCESS {
            sync_stream_on_error(&self.stream, "voxel_bake_trace_columns launch error");
            return Err(format!(
                "cuLaunchKernel(voxel_bake_trace_columns) failed ({rc_trace:?})"
            ));
        }

        let mut merge_args: [*mut c_void; 7] = [
            (&raw mut p_grid).cast(),
            (&raw mut p_raw_spans).cast(),
            (&raw mut p_raw_counts).cast(),
            (&raw mut p_coords).cast(),
            (&raw mut p_out_spans).cast(),
            (&raw mut p_out_counts).cast(),
            (&raw mut p_out_overflow).cast(),
        ];

        // SAFETY: `self.merge_fn` is `voxel_bake_merge_columns` from `voxel_bake.cuh`, which
        // accepts these exact 7 device-pointer parameters ordered strictly after
        // `voxel_bake_trace_columns` on `self.stream`. Every buffer is live and sized to the
        // capacities encoded in `input.grid`, and `self.stream` is synchronized before any buffer
        // is read or dropped.
        let rc_merge = unsafe {
            cu::cuLaunchKernel(
                self.merge_fn,
                blocks,
                1,
                1,
                BLOCK_SIZE,
                1,
                1,
                0,
                self.stream.cu_stream(),
                merge_args.as_mut_ptr(),
                std::ptr::null_mut(),
            )
        };
        if rc_merge != cu::cudaError_enum_CUDA_SUCCESS {
            sync_stream_on_error(&self.stream, "voxel_bake_merge_columns launch error");
            return Err(format!(
                "cuLaunchKernel(voxel_bake_merge_columns) failed ({rc_merge:?})"
            ));
        }

        self.stream
            .synchronize()
            .map_err(|e| format!("CUDA voxel bake stream synchronize failed: {e}"))?;

        let raw_ovf = self.buffers.raw_overflow.download_u32("overflow_count")?;
        let out_ovf = self
            .buffers
            .out_overflow
            .download_u32("out_overflow_count")?;
        if raw_ovf > 0 || out_ovf > 0 {
            return Err(format!(
                "GPU voxel bake overflow (raw={raw_ovf}, merged={out_ovf})"
            ));
        }

        self.buffers.out_spans.download_into(
            "out_spans",
            out_spans_elems,
            &mut self.buffers.host_out_spans,
        )?;
        self.buffers.out_counts.download_into(
            "out_counts",
            total_cols,
            &mut self.buffers.host_out_counts,
        )?;

        let (col_offsets, spans) = VoxelVolume::unpack_gpu_baked_columns(
            &self.buffers.host_out_spans,
            &self.buffers.host_out_counts,
            out_cap,
        )
        .map_err(|e| format!("GPU voxel bake column unpack failed: {e}"))?;
        VoxelVolume::from_baked_columns(input.bounds, None, nx, nz, col_offsets, spans, *cfg)
            .map_err(|e| format!("GPU voxel volume construction failed: {e}"))
    }
}

impl Drop for GpuVoxelBaker {
    fn drop(&mut self) {
        if !self.module.is_null() && self.cuda.bind_to_thread().is_ok() {
            if let Err(err) = self.stream.synchronize() {
                eprintln!("nv-demo: GpuVoxelBaker stream synchronize on drop failed ({err})");
            }
            unload_module_best_effort(self.module, "drop");
            self.module = std::ptr::null_mut();
        }
    }
}

/// Unloads a loaded `CUmodule` on an error or drop path without panicking.
fn unload_module_best_effort(module: cu::CUmodule, context: &str) {
    // SAFETY: `module` was created via `cuModuleLoadData` on the currently bound CUDA context and
    // is unloaded at most once.
    let rc = unsafe { cu::cuModuleUnload(module) };
    if rc != cu::cudaError_enum_CUDA_SUCCESS {
        eprintln!("nv-demo: GpuVoxelBaker cuModuleUnload({context}) failed ({rc:?})");
    }
}

/// Synchronizes `stream` before dropping local device allocations on a mid-bake error path.
fn sync_stream_on_error(stream: &Stream, context: &str) {
    if let Err(err) = stream.synchronize() {
        eprintln!("nv-demo: GpuVoxelBaker stream synchronize after {context} failed ({err})");
    }
}

/// Computes `count * size_of::<T>()` with explicit overflow checking.
fn checked_bytes<T>(count: usize, label: &str) -> Result<usize, String> {
    count.checked_mul(std::mem::size_of::<T>()).ok_or_else(|| {
        format!(
            "GPU voxel bake '{label}' byte size overflow ({count} x {} B)",
            std::mem::size_of::<T>()
        )
    })
}

#[cfg(test)]
mod tests {
    use csg_core::{CsgNode, Mat3x4, MaterialId, SemanticId, Vec3, semantic};
    use csg_scene::voxel_volume::{COARSE_VOXEL_M, VoxelVolume, VoxelVolumeConfig};
    use csg_scene::{Assembly, Instance, WorldScene, inst_flags};

    use super::GpuVoxelBaker;

    fn add_box(
        world: &mut WorldScene,
        name: &str,
        min: Vec3,
        max: Vec3,
        sem: SemanticId,
    ) {
        let half = (max - min) * 0.5;
        let center = (min + max) * 0.5;
        let node = CsgNode::box3([half.x, half.y, half.z], MaterialId(1), sem);
        let lods = world.library.compile_lods(&node, &[0.0]);
        let asm = Assembly::from_compiled(name, lods, &world.library);
        let id = world.register_assembly(asm);
        let mut inst = Instance::new(id, Mat3x4::translate(center));
        inst.flags = inst_flags::DEFAULT_STATIC;
        world.add_static(inst);
    }

    #[test]
    #[ignore = "requires CUDA runtime"]
    fn gpu_voxel_baker_matches_cpu_voxel_volume_on_synthetic_scene() {
        let mut world = WorldScene::new(32.0);
        add_box(
            &mut world,
            "ground",
            Vec3::new(0.0, -4.0, 0.0),
            Vec3::new(24.0, 0.0, 24.0),
            semantic::GROUND,
        );
        add_box(
            &mut world,
            "road",
            Vec3::new(8.0, -1.0, 0.0),
            Vec3::new(16.0, 1.0, 24.0),
            semantic::ROAD,
        );
        add_box(
            &mut world,
            "building_west",
            Vec3::new(1.0, 0.0, 4.0),
            Vec3::new(7.0, 12.0, 20.0),
            semantic::BUILDING,
        );
        add_box(
            &mut world,
            "building_east",
            Vec3::new(17.0, 0.0, 4.0),
            Vec3::new(23.0, 10.0, 20.0),
            semantic::BUILDING,
        );
        add_box(
            &mut world,
            "water",
            Vec3::new(24.0, -4.0, 0.0),
            Vec3::new(36.0, 0.0, 24.0),
            semantic::WATER,
        );
        add_box(
            &mut world,
            "bridge",
            Vec3::new(24.0, 6.0, 8.0),
            Vec3::new(36.0, 7.0, 16.0),
            semantic::ROAD,
        );

        let mut baker = GpuVoxelBaker::new().expect("GpuVoxelBaker::new must succeed with CUDA");

        // 1. Coarse axis-aligned build: GPU and CPU spans and ESDF must match bit-for-bit,
        //    including identical BLAKE3 hashes.
        let coarse_cfg = VoxelVolumeConfig {
            ceiling_margin_m: 12.0,
            adaptive_refinement: false,
            ..VoxelVolumeConfig::default()
        };
        let cpu_coarse = VoxelVolume::build(&world, None, coarse_cfg);
        let gpu_coarse = baker
            .bake_voxel_volume(&world, &coarse_cfg)
            .expect("GPU coarse voxel bake must succeed");

        assert_eq!(gpu_coarse.bounds(), cpu_coarse.bounds());
        assert_eq!(gpu_coarse.grid_dims(), cpu_coarse.grid_dims());
        assert_eq!(
            gpu_coarse.hash(),
            cpu_coarse.hash(),
            "axis-aligned GPU and CPU VoxelVolumes must yield identical BLAKE3 hashes"
        );

        let (nx, ny, nz) = cpu_coarse.grid_dims();
        let b = cpu_coarse.bounds();
        for iz in 0..nz {
            let z = b.min.z + (iz as f32 + 0.5) * COARSE_VOXEL_M;
            for ix in 0..nx {
                let x = b.min.x + (ix as f32 + 0.5) * COARSE_VOXEL_M;
                assert_eq!(
                    gpu_coarse.column_spans(x, z),
                    cpu_coarse.column_spans(x, z),
                    "column_spans mismatch at ({ix}, {iz})"
                );
            }
        }

        for iz in (0..nz).step_by(2) {
            for iy in (0..ny).step_by(2) {
                for ix in (0..nx).step_by(2) {
                    let p = Vec3::new(
                        b.min.x + ix as f32 + 0.37,
                        b.min.y + iy as f32 + 0.61,
                        b.min.z + iz as f32 + 0.23,
                    );
                    let diff = (gpu_coarse.distance(p) - cpu_coarse.distance(p)).abs();
                    assert!(
                        diff < 1.0e-4,
                        "distance mismatch at {p:?}: gpu={} cpu={} (diff={diff})",
                        gpu_coarse.distance(p),
                        cpu_coarse.distance(p)
                    );
                }
            }
        }

        // 2. Default adaptive config (exact config used by CsgSceneOracle): bounds, column_spans,
        //    and ESDF distance must match cleanly.
        let default_cfg = VoxelVolumeConfig::default();
        let cpu_def = VoxelVolume::build(&world, None, default_cfg);
        let gpu_def = baker
            .bake_voxel_volume(&world, &default_cfg)
            .expect("GPU default adaptive voxel bake must succeed");
        assert_eq!(gpu_def.bounds(), cpu_def.bounds());
        for iz in 0..nz {
            let z = cpu_def.bounds().min.z + (iz as f32 + 0.5) * COARSE_VOXEL_M;
            for ix in 0..nx {
                let x = cpu_def.bounds().min.x + (ix as f32 + 0.5) * COARSE_VOXEL_M;
                assert_eq!(
                    gpu_def.column_spans(x, z),
                    cpu_def.column_spans(x, z),
                    "adaptive column_spans mismatch at ({ix}, {iz})"
                );
            }
        }
        for iz in (0..nz).step_by(3) {
            for iy in (0..18).step_by(3) {
                for ix in (0..nx).step_by(3) {
                    let p = Vec3::new(
                        cpu_def.bounds().min.x + ix as f32 + 0.25,
                        cpu_def.bounds().min.y + iy as f32 + 0.5,
                        cpu_def.bounds().min.z + iz as f32 + 0.75,
                    );
                    let diff = (gpu_def.distance(p) - cpu_def.distance(p)).abs();
                    assert!(
                        diff < 1.0e-3,
                        "adaptive distance mismatch at {p:?}: gpu={} cpu={} (diff={diff})",
                        gpu_def.distance(p),
                        cpu_def.distance(p)
                    );
                }
            }
        }
    }
}
