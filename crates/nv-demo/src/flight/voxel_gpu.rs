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

/// Reusable CUDA driver baker executing `csg-scene`'s two-pass `voxel_bake.cuh` kernels.
///
/// Why: holding the primary CUDA context, a dedicated stream, and the disk-cached `CUmodule`
/// across Auto-Plan jobs amortizes NVRTC compilation and module loading to < 5 ms on warm runs
/// while keeping the background `"nv-flight-planner"` thread isolated from the OptiX render
/// stream.
pub struct GpuVoxelBaker {
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
        let d_grid = upload_or_dummy(&self.stream, "grid", &grid_slice)?;
        let d_items = upload_or_dummy(&self.stream, "items", &input.items)?;
        let d_unbounded = upload_or_dummy(&self.stream, "unbounded_items", &input.unbounded_items)?;
        let d_tile_starts = upload_or_dummy(&self.stream, "tile_starts", &input.tile_starts)?;
        let d_tile_items = upload_or_dummy(&self.stream, "tile_items", &input.tile_items)?;

        let d_compounds = upload_or_dummy(&self.stream, "compounds", &lib.compounds)?;
        let d_ops = upload_or_dummy(&self.stream, "ops", &lib.ops)?;
        let d_prims = upload_or_dummy(&self.stream, "prims", &lib.prims)?;
        let d_aux = upload_or_dummy(&self.stream, "aux_planes", &lib.aux_planes)?;
        let d_guards = upload_or_dummy(&self.stream, "guard_aabbs", &lib.guard_aabbs)?;
        let d_repeats = upload_or_dummy(&self.stream, "repeats", &lib.repeats)?;

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

        let d_raw_spans = alloc_uninit_bytes(&self.stream, "raw_spans", raw_spans_bytes)?;
        let d_raw_counts = alloc_uninit_bytes(&self.stream, "raw_counts", raw_counts_bytes)?;
        let d_coords = alloc_uninit_bytes(&self.stream, "coord_buf", coord_bytes)?;
        let d_out_spans = alloc_uninit_bytes(&self.stream, "out_spans", out_spans_bytes)?;
        let d_out_counts = alloc_uninit_bytes(&self.stream, "out_counts", out_counts_bytes)?;
        let d_raw_overflow = alloc_uninit_bytes(&self.stream, "overflow_count", u32_bytes)?;
        let d_out_overflow = alloc_uninit_bytes(&self.stream, "out_overflow_count", u32_bytes)?;

        memset_zero_async(&self.stream, "raw_counts", &d_raw_counts, raw_counts_bytes)?;
        memset_zero_async(&self.stream, "out_counts", &d_out_counts, out_counts_bytes)?;
        memset_zero_async(&self.stream, "overflow_count", &d_raw_overflow, u32_bytes)?;
        memset_zero_async(
            &self.stream,
            "out_overflow_count",
            &d_out_overflow,
            u32_bytes,
        )?;

        let blocks = total_cols_u32.div_ceil(BLOCK_SIZE);

        let mut p_grid = d_grid.cu_deviceptr();
        let mut p_items = d_items.cu_deviceptr();
        let mut p_unbounded = d_unbounded.cu_deviceptr();
        let mut p_tile_starts = d_tile_starts.cu_deviceptr();
        let mut p_tile_items = d_tile_items.cu_deviceptr();
        let mut p_compounds = d_compounds.cu_deviceptr();
        let mut p_ops = d_ops.cu_deviceptr();
        let mut p_prims = d_prims.cu_deviceptr();
        let mut p_aux = d_aux.cu_deviceptr();
        let mut p_guards = d_guards.cu_deviceptr();
        let mut p_repeats = d_repeats.cu_deviceptr();
        let mut p_raw_spans = d_raw_spans.cu_deviceptr();
        let mut p_raw_counts = d_raw_counts.cu_deviceptr();
        let mut p_raw_overflow = d_raw_overflow.cu_deviceptr();
        let mut p_coords = d_coords.cu_deviceptr();
        let mut p_out_spans = d_out_spans.cu_deviceptr();
        let mut p_out_counts = d_out_counts.cu_deviceptr();
        let mut p_out_overflow = d_out_overflow.cu_deviceptr();

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

        let raw_ovf = download_u32("overflow_count", &d_raw_overflow)?;
        let out_ovf = download_u32("out_overflow_count", &d_out_overflow)?;
        if raw_ovf > 0 || out_ovf > 0 {
            return Err(format!(
                "GPU voxel bake overflow (raw={raw_ovf}, merged={out_ovf})"
            ));
        }

        let out_spans: Vec<ColumnSpanPod> =
            download_vec("out_spans", &d_out_spans, out_spans_elems)?;
        let out_counts: Vec<u32> = download_vec("out_counts", &d_out_counts, total_cols)?;

        let (col_offsets, spans) =
            VoxelVolume::unpack_gpu_baked_columns(&out_spans, &out_counts, out_cap)
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

/// Allocates `bytes` (`>= 1`) of uninitialized stream-ordered device memory on `stream`.
fn alloc_uninit_bytes(
    stream: &Arc<Stream>,
    label: &str,
    bytes: usize,
) -> Result<DeviceBuffer<u8>, String> {
    if bytes == 0 {
        return Err(format!(
            "GPU voxel bake '{label}' allocation size must be > 0"
        ));
    }
    // SAFETY: every byte read by device kernels or host readback from this allocation is first
    // written on `stream` via `cuMemcpyHtoDAsync_v2`, `cuMemsetD8Async`, or kernel outputs, and
    // `stream` is synchronized before the buffer is dropped.
    unsafe { DeviceBuffer::<u8>::uninitialized_async(stream, bytes) }
        .map_err(|e| format!("CUDA alloc '{label}' ({bytes} B) failed: {e}"))
}

/// Uploads `slice` asynchronously on `stream`, allocating a 1-element zeroed dummy buffer when
/// `slice` is empty so kernel device pointers are always non-null.
fn upload_or_dummy<T: Pod>(
    stream: &Arc<Stream>,
    label: &str,
    slice: &[T],
) -> Result<DeviceBuffer<u8>, String> {
    if slice.is_empty() {
        let dummy_bytes = std::mem::size_of::<T>().max(1);
        return DeviceBuffer::<u8>::zeroed(stream, dummy_bytes)
            .map_err(|e| format!("CUDA dummy alloc '{label}' ({dummy_bytes} B) failed: {e}"));
    }
    let bytes: &[u8] = bytemuck::cast_slice(slice);
    let buf = alloc_uninit_bytes(stream, label, bytes.len())?;
    // SAFETY: `buf` owns `bytes.len()` 256-byte-aligned device bytes allocated on `stream`, and
    // `bytes` points to valid initialized `Pod` host memory that stays live across the call.
    let rc = unsafe {
        cu::cuMemcpyHtoDAsync_v2(
            buf.cu_deviceptr(),
            bytes.as_ptr().cast(),
            bytes.len(),
            stream.cu_stream(),
        )
    };
    if rc != cu::cudaError_enum_CUDA_SUCCESS {
        return Err(format!("cuMemcpyHtoDAsync_v2('{label}') failed ({rc:?})"));
    }
    Ok(buf)
}

/// Asynchronously zeroes `bytes` of device buffer `buf` on `stream` via `cuMemsetD8Async`.
fn memset_zero_async(
    stream: &Stream,
    label: &str,
    buf: &DeviceBuffer<u8>,
    bytes: usize,
) -> Result<(), String> {
    // SAFETY: `buf` was allocated with at least `bytes` valid device bytes on `stream`.
    let rc = unsafe { cu::cuMemsetD8Async(buf.cu_deviceptr(), 0u8, bytes, stream.cu_stream()) };
    if rc != cu::cudaError_enum_CUDA_SUCCESS {
        return Err(format!("cuMemsetD8Async('{label}') failed ({rc:?})"));
    }
    Ok(())
}

/// Downloads a single `u32` from `buf` after `self.stream` has been synchronized.
fn download_u32(label: &str, buf: &DeviceBuffer<u8>) -> Result<u32, String> {
    let mut val = 0u32;
    // SAFETY: the caller already synchronized `stream`, `buf` holds at least `size_of::<u32>()`
    // valid device bytes, and `val` is a valid writable `u32` on the stack.
    let rc = unsafe {
        cu::cuMemcpyDtoH_v2(
            (&raw mut val).cast(),
            buf.cu_deviceptr(),
            std::mem::size_of::<u32>(),
        )
    };
    if rc != cu::cudaError_enum_CUDA_SUCCESS {
        return Err(format!("cuMemcpyDtoH_v2('{label}') failed ({rc:?})"));
    }
    Ok(val)
}

/// Downloads `count` elements of `T: Pod` from `buf` after `self.stream` has been synchronized.
fn download_vec<T: Pod>(label: &str, buf: &DeviceBuffer<u8>, count: usize) -> Result<Vec<T>, String> {
    let bytes = checked_bytes::<T>(count, label)?;
    let mut out = vec![T::zeroed(); count];
    if bytes == 0 {
        return Ok(out);
    }
    let raw: &mut [u8] = bytemuck::cast_slice_mut(&mut out);
    // SAFETY: the caller already synchronized `stream`, `buf` holds at least `raw.len()` valid
    // device bytes, and `raw` points to `raw.len()` writable `Pod` host bytes.
    let rc = unsafe { cu::cuMemcpyDtoH_v2(raw.as_mut_ptr().cast(), buf.cu_deviceptr(), raw.len()) };
    if rc != cu::cudaError_enum_CUDA_SUCCESS {
        return Err(format!("cuMemcpyDtoH_v2('{label}') failed ({rc:?})"));
    }
    Ok(out)
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
