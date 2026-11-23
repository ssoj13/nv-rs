//! CUDA/NVRTC GPU-direct articulated crowd simulation backend for `csg-actors`.
//!
//! What: compiles `csg_actors::ACTOR_SIM_CUH` (`step_and_emit_actors_cuda`) alongside a compact
//! bounds-checked scatter kernel (`scatter_crowd_to_csg_and_optix`) via NVRTC, steps `ActorStatePod`
//! LIPM/PD balance + rigid FK + analytical 2-bone limb IK on the GPU, and writes resolved transforms
//! directly into device `CsgGpuInstance`, `prev_xforms`, and OptiX `OptixInstance` VRAM buffers.
//! Why: scales articulated crowd simulation to 2,000+ actors in sub-millisecond GPU time with zero
//! steady-state VRAM reallocations and zero per-frame CPU bone readback, while keeping explicit
//! `CrowdBackend::Cuda` strict (never silently downgrading on failure).
//! Used by: `crate::renderer`, `crate::csg_scene_host`.

use std::ffi::{CString, c_void};
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Instant;

use bytemuck::{Pod, Zeroable};
use cuda_bindings as cu;
use cuda_core::{Context as CudaContext, DeviceBuffer, Stream};

use csg_actors::{
    ACTOR_SIM_CUH, ActorDescPod, ActorStatePod, ArchetypeKind, ArchetypeMetaPod, BoneDescPod,
    BoneOutPod, GroundSamplePod, IntentPod, SimUniformsPod, intent_flags,
};
use csg_scene::{CROWD_SIDEWALK_CLEARANCE_M, SceneBuilt, inst_flags};
#[cfg(test)]
use csg_scene::encode_walkway_target;

use crate::csg_scene_host::CSG_SBT_PRIM_OFFSET;

/// Extra `CsgGpuInstance::flags` bits OR'd onto visible scattered crowd bone instances so shadows
/// and `Motion` AOV optical flow match CPU-packed dynamic actors identically.
const SCATTER_VISIBLE_EXTRA_FLAGS: u32 =
    inst_flags::DYNAMIC | inst_flags::CAST_SHADOW | inst_flags::MOTION_BLUR;

/// OptiX instance flag disabling any-hit programs on opaque crowd bone instances.
const OPTIX_INST_DISABLE_ANYHIT: u32 =
    optix::sys::OptixInstanceFlags_OPTIX_INSTANCE_FLAG_DISABLE_ANYHIT as u32;

const _: () = assert!(OPTIX_INST_DISABLE_ANYHIT == 4);
const _: () = assert!(CSG_SBT_PRIM_OFFSET == 10);

/// Maximum actor root displacement (in meters) before forcing a full OptiX IAS rebuild instead of
/// an incremental `OPTIX_BUILD_OPERATION_UPDATE` refit (preserves tight BVH traversal quality).
pub const CUDA_IAS_REBUILD_DISP_M: f32 = 8.0;

/// GPU DEM elevation header (`32 B`, 16-byte aligned) matching `ActorDemUniformsPod` in CUDA.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
struct DemUniformsPod {
    origin_x: f32,
    origin_z: f32,
    cell_x: f32,
    cell_z: f32,
    nx: u32,
    nz: u32,
    flat_y: f32,
    offset_m: f32,
}

const _: () = assert!(std::mem::size_of::<DemUniformsPod>() == 32);

/// Page-locked 4-byte host `u32` for zero-sync asynchronous GPU displacement flag readback.
struct PinnedU32 {
    ptr: *mut u32,
    cuda: Arc<CudaContext>,
}

// SAFETY: Page-locked CUDA host memory is context-owned and serialized through `&mut CudaCrowdBackend`.
unsafe impl Send for PinnedU32 {}

impl PinnedU32 {
    fn new(cuda: &Arc<CudaContext>) -> Result<Self, String> {
        cuda.bind_to_thread()
            .map_err(|e| format!("CUDA bind failed for PinnedU32: {e}"))?;
        let mut raw: *mut c_void = std::ptr::null_mut();
        // SAFETY: `cuda` is current on this thread and `raw` is a valid out-pointer.
        let rc = unsafe { cu::cuMemAllocHost_v2(&mut raw, std::mem::size_of::<u32>()) };
        if rc != cu::cudaError_enum_CUDA_SUCCESS || raw.is_null() {
            return Err(format!("cuMemAllocHost_v2(4 B) failed ({rc:?})"));
        }
        let ptr = raw.cast::<u32>();
        // SAFETY: `ptr` points to 4 writable aligned host bytes owned by `Self`.
        unsafe {
            ptr.write(0u32);
        }
        Ok(Self {
            ptr,
            cuda: cuda.clone(),
        })
    }

    #[inline]
    fn write(&mut self, val: u32) {
        // SAFETY: `self.ptr` is valid for the lifetime of `Self` and uniquely borrowed via `&mut self`.
        unsafe {
            std::ptr::write_volatile(self.ptr, val);
        }
    }

    #[inline]
    fn read(&self) -> u32 {
        // SAFETY: `self.ptr` is valid for the lifetime of `Self`; caller ensures prior async DMA completed.
        unsafe { std::ptr::read_volatile(self.ptr) }
    }

    #[inline]
    fn as_mut_c_void(&mut self) -> *mut c_void {
        self.ptr.cast()
    }
}

impl Drop for PinnedU32 {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.cuda.bind_to_thread().is_ok() {
            // SAFETY: `self.ptr` was allocated via `cuMemAllocHost_v2` on `self.cuda`.
            let rc = unsafe { cu::cuMemFreeHost(self.ptr.cast()) };
            if rc != cu::cudaError_enum_CUDA_SUCCESS {
                eprintln!("nv-demo: PinnedU32 cuMemFreeHost failed ({rc:?})");
            }
            self.ptr = std::ptr::null_mut();
        }
    }
}

/// GPU steering, DEM ground sampling, bounds-checked CSG/OptiX scatter, and amortized IAS
/// displacement-flag kernels appended to `csg_actors::ACTOR_SIM_CUH` in the same NVRTC unit.
const SCATTER_KERNEL_CU: &str = r#"
#if defined(__CUDACC__) || defined(__CUDACC_RTC__)
struct alignas(16) ActorDemUniformsPod {
    float origin_x;
    float origin_z;
    float cell_x;
    float cell_z;
    uint32_t nx;
    uint32_t nz;
    float flat_y;
    float offset_m;
};
static_assert(sizeof(ActorDemUniformsPod) == 32, "ActorDemUniformsPod must be 32 bytes");

__device__ __forceinline__ float crowd_dem_get(
    const float* __restrict__ heights,
    uint32_t nx,
    uint32_t nz,
    uint32_t ix,
    uint32_t iz) {
    if (heights == nullptr || nx == 0u || nz == 0u) {
        return 0.0f;
    }
    const uint32_t cx = (ix < nx) ? ix : (nx - 1u);
    const uint32_t cz = (iz < nz) ? iz : (nz - 1u);
    const float v = heights[(size_t)cz * (size_t)nx + (size_t)cx];
    return isfinite(v) ? v : 0.0f;
}

__device__ __forceinline__ float crowd_dem_sample(
    const ActorDemUniformsPod& dem,
    const float* __restrict__ heights,
    float x,
    float z) {
    if (heights == nullptr || dem.nx < 2u || dem.nz < 2u
        || !isfinite(x) || !isfinite(z)
        || !(dem.cell_x > 1e-6f) || !(dem.cell_z > 1e-6f)) {
        return (isfinite(dem.flat_y) ? dem.flat_y : 0.0f) + dem.offset_m;
    }
    const float fx = actor_clampf((x - dem.origin_x) / dem.cell_x, 0.0f, (float)(dem.nx - 1u));
    const float fz = actor_clampf((z - dem.origin_z) / dem.cell_z, 0.0f, (float)(dem.nz - 1u));
    uint32_t ix = (uint32_t)floorf(fx);
    uint32_t iz = (uint32_t)floorf(fz);
    if (ix > dem.nx - 2u) ix = dem.nx - 2u;
    if (iz > dem.nz - 2u) iz = dem.nz - 2u;
    const float u = actor_clampf(fx - (float)ix, 0.0f, 1.0f);
    const float v = actor_clampf(fz - (float)iz, 0.0f, 1.0f);
    const float h00 = crowd_dem_get(heights, dem.nx, dem.nz, ix, iz);
    const float h10 = crowd_dem_get(heights, dem.nx, dem.nz, ix + 1u, iz);
    const float h01 = crowd_dem_get(heights, dem.nx, dem.nz, ix, iz + 1u);
    const float h11 = crowd_dem_get(heights, dem.nx, dem.nz, ix + 1u, iz + 1u);
    const float a_u = h10 - h00;
    const float a_v = h01 - h00;
    const float a_uv = h00 - h10 - h01 + h11;
    return (h00 + a_u * u + a_v * v + a_uv * u * v) + dem.offset_m;
}

__device__ __forceinline__ float3 crowd_dem_normal(
    const ActorDemUniformsPod& dem,
    const float* __restrict__ heights,
    float x,
    float z) {
    if (heights == nullptr || dem.nx < 2u || dem.nz < 2u
        || !isfinite(x) || !isfinite(z)
        || !(dem.cell_x > 1e-6f) || !(dem.cell_z > 1e-6f)) {
        return actor_v3(0.0f, 1.0f, 0.0f);
    }
    const float fx = actor_clampf((x - dem.origin_x) / dem.cell_x, 0.0f, (float)(dem.nx - 1u));
    const float fz = actor_clampf((z - dem.origin_z) / dem.cell_z, 0.0f, (float)(dem.nz - 1u));
    uint32_t ix = (uint32_t)floorf(fx);
    uint32_t iz = (uint32_t)floorf(fz);
    if (ix > dem.nx - 2u) ix = dem.nx - 2u;
    if (iz > dem.nz - 2u) iz = dem.nz - 2u;
    const float u = actor_clampf(fx - (float)ix, 0.0f, 1.0f);
    const float v = actor_clampf(fz - (float)iz, 0.0f, 1.0f);
    const float h00 = crowd_dem_get(heights, dem.nx, dem.nz, ix, iz);
    const float h10 = crowd_dem_get(heights, dem.nx, dem.nz, ix + 1u, iz);
    const float h01 = crowd_dem_get(heights, dem.nx, dem.nz, ix, iz + 1u);
    const float h11 = crowd_dem_get(heights, dem.nx, dem.nz, ix + 1u, iz + 1u);
    const float a_u = h10 - h00;
    const float a_v = h01 - h00;
    const float a_uv = h00 - h10 - h01 + h11;
    const float dh_dx = (a_u + a_uv * v) / dem.cell_x;
    const float dh_dz = (a_v + a_uv * u) / dem.cell_z;
    return actor_norm_or(actor_v3(-dh_dx, 1.0f, -dh_dz), actor_v3(0.0f, 1.0f, 0.0f));
}

extern "C" __global__ void prepare_crowd_steering_and_grounds(
    uint32_t actor_count,
    uint32_t quadruped_arch_id,
    const ActorStatePod* __restrict__ states,
    const ActorDescPod* __restrict__ descs,
    ActorIntentPod* __restrict__ intents,
    ActorGroundSamplePod* __restrict__ grounds,
    const ActorDemUniformsPod* __restrict__ dem_ptr,
    const float* __restrict__ dem_heights,
    const float2* __restrict__ walkway_pts,
    const uint2* __restrict__ walkway_spans,
    uint32_t num_walkways,
    uint32_t* __restrict__ disp_flag) {
    const uint32_t i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i == 0u && disp_flag != nullptr) {
        *disp_flag = 0u;
    }
    if (i >= actor_count) {
        return;
    }

    const ActorStatePod st = states[i];
    const ActorDescPod desc = descs[i];
    const float pos_x = st.pos_yaw[0];
    const float pos_z = st.pos_yaw[2];

    // 1. Update walkway patrol steering intent in-place on GPU (matches `SceneBuilt::step_crowd`).
    if (num_walkways > 0u && walkway_pts != nullptr && walkway_spans != nullptr) {
        ActorIntentPod intent = intents[i];
        if ((intent.flags & ACTOR_INTENT_FREEZE_POSE) == 0u && intent.target_id != 0xFFFFFFFFu) {
            const uint32_t w_idx = (intent.target_id >> 16u) & 0x7FFFu;
            uint32_t wp_idx = (intent.target_id >> 1u) & 0x7FFFu;
            bool reverse = (intent.target_id & 1u) != 0u;
            if (w_idx < num_walkways) {
                const uint2 span = walkway_spans[w_idx];
                const uint32_t start = span.x;
                const uint32_t len = span.y;
                if (len >= 2u) {
                    const uint32_t last_idx = len - 1u;
                    if (wp_idx > last_idx) wp_idx = last_idx;
                    float2 wp = walkway_pts[start + wp_idx];
                    float dx = wp.x - pos_x;
                    float dz = wp.y - pos_z;
                    if (dx * dx + dz * dz < 0.75f * 0.75f) {
                        if (!reverse) {
                            if (wp_idx < last_idx) {
                                wp_idx += 1u;
                            } else {
                                reverse = true;
                                wp_idx = (last_idx > 0u) ? (last_idx - 1u) : 0u;
                            }
                        } else if (wp_idx > 0u) {
                            wp_idx -= 1u;
                        } else {
                            reverse = false;
                            wp_idx = (1u < last_idx) ? 1u : last_idx;
                        }
                        wp = walkway_pts[start + wp_idx];
                        dx = wp.x - pos_x;
                        dz = wp.y - pos_z;
                    }
                    const float dist = sqrtf(dx * dx + dz * dz);
                    if (isfinite(dist) && dist > 1e-3f) {
                        const float speed = (isfinite(intent.move_cmd[3]) && intent.move_cmd[3] > 0.1f)
                            ? actor_clampf(intent.move_cmd[3], 0.45f, 3.2f)
                            : 1.25f;
                        const float inv_d = 1.0f / dist;
                        intent.move_cmd[0] = dx * inv_d * speed;
                        intent.move_cmd[1] = dz * inv_d * speed;
                        intent.move_cmd[2] = atan2f(dx, dz);
                        intent.move_cmd[3] = speed;
                        intent.flags |= ACTOR_INTENT_MASK_VEL | ACTOR_INTENT_MASK_HEADING;
                    }
                    if (w_idx < 32767u && wp_idx < 32767u) {
                        intent.target_id = (w_idx << 16u) | (wp_idx << 1u) | (reverse ? 1u : 0u);
                    } else {
                        intent.target_id = 0xFFFFFFFFu;
                    }
                    intents[i] = intent;
                }
            }
        }
    }

    // 2. Sample DEM height + normal + 4 nominal foot heights directly into VRAM.
    const ActorDemUniformsPod dem = *dem_ptr;
    const float hy = crowd_dem_sample(dem, dem_heights, pos_x, pos_z);
    const float3 nrm = crowd_dem_normal(dem, dem_heights, pos_x, pos_z);
    const float yaw = st.pos_yaw[3];
    const float sy = sinf(yaw);
    const float cy = cosf(yaw);
    const float hs = fmaxf(desc.scale_dims[0], 0.25f);
    float ox[4];
    float oz[4];
    if (desc.archetype_id == quadruped_arch_id) {
        ox[0] = -0.11f * cy + 0.32f * hs * sy; oz[0] =  0.11f * sy + 0.32f * hs * cy;
        ox[1] =  0.11f * cy + 0.32f * hs * sy; oz[1] = -0.11f * sy + 0.32f * hs * cy;
        ox[2] = -0.11f * cy - 0.26f * hs * sy; oz[2] =  0.11f * sy - 0.26f * hs * cy;
        ox[3] =  0.11f * cy - 0.26f * hs * sy; oz[3] = -0.11f * sy - 0.26f * hs * cy;
    } else {
        ox[0] = -0.22f * cy; oz[0] =  0.22f * sy;
        ox[1] =  0.22f * cy; oz[1] = -0.22f * sy;
        ox[2] = -0.11f * cy; oz[2] =  0.11f * sy;
        ox[3] =  0.11f * cy; oz[3] = -0.11f * sy;
    }
    ActorGroundSamplePod out_g;
    out_g.height_normal[0] = hy;
    out_g.height_normal[1] = nrm.x;
    out_g.height_normal[2] = nrm.y;
    out_g.height_normal[3] = nrm.z;
    for (int k = 0; k < 4; ++k) {
        out_g.foot_heights[k] = crowd_dem_sample(dem, dem_heights, pos_x + ox[k], pos_z + oz[k]);
    }
    grounds[i] = out_g;
}

extern "C" __global__ void check_crowd_ias_displacement(
    uint32_t actor_count,
    const ActorStatePod* __restrict__ states,
    float4* __restrict__ rebuild_pos,
    float max_disp_sq,
    uint32_t reset_anchors,
    uint32_t* __restrict__ disp_flag) {
    const uint32_t i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= actor_count) {
        return;
    }
    const ActorStatePod st = states[i];
    if (reset_anchors != 0u) {
        rebuild_pos[i] = make_float4(st.pos_yaw[0], st.pos_yaw[1], st.pos_yaw[2], 0.0f);
        return;
    }
    const float4 prev = rebuild_pos[i];
    const float dx = st.pos_yaw[0] - prev.x;
    const float dy = st.pos_yaw[1] - prev.y;
    const float dz = st.pos_yaw[2] - prev.z;
    const float d2 = dx * dx + dy * dy + dz * dz;
    if (isfinite(d2) && d2 > max_disp_sq) {
        atomicOr(disp_flag, 1u);
    }
}

extern "C" __global__ void scatter_crowd_to_csg_and_optix(
    const ActorBoneOutPod* __restrict__ src_bones,
    const float* __restrict__ src_prev_xforms,
    const uint32_t* __restrict__ bone_slot_map,
    uint32_t total_bones,
    uint32_t max_csg_instances,
    uint32_t extra_csg_flags,
    ActorBoneOutPod* __restrict__ dst_csg_instances,
    float* __restrict__ dst_csg_prev_xforms,
    ActorOptixInstancePod* __restrict__ dst_optix_instances,
    uint32_t optix_slot_offset,
    uint32_t max_optix_instances,
    const uint64_t* __restrict__ compound_gas_handles,
    const uint32_t* __restrict__ compound_sbt_offsets,
    uint32_t compound_count) {
    const uint32_t gidx = blockIdx.x * blockDim.x + threadIdx.x;
    if (gidx >= total_bones) {
        return;
    }
    const uint32_t out_idx = bone_slot_map[gidx];
    if (out_idx == 0xFFFFFFFFu || out_idx >= max_csg_instances) {
        return;
    }

    ActorBoneOutPod pod = src_bones[gidx];
    const bool visible = (pod.flags & ACTOR_BONE_OUT_VISIBLE) != 0u;
    if (visible) {
        pod.flags |= extra_csg_flags;
    }
    dst_csg_instances[out_idx] = pod;

    if (dst_csg_prev_xforms != nullptr && src_prev_xforms != nullptr) {
        const float* sp = src_prev_xforms + (size_t)gidx * 12u;
        float* dp = dst_csg_prev_xforms + (size_t)out_idx * 12u;
        for (int k = 0; k < 12; ++k) {
            dp[k] = sp[k];
        }
    }

    if (dst_optix_instances != nullptr) {
        const uint32_t oi_idx = optix_slot_offset + out_idx;
        if (oi_idx < max_optix_instances) {
            ActorOptixInstancePod oi;
            for (int k = 0; k < 12; ++k) {
                oi.transform[k] = pod.xform[k];
            }
            oi.instanceId = out_idx;
            const uint32_t cid = pod.compound_id;
            oi.sbtOffset = (compound_sbt_offsets != nullptr && cid < compound_count)
                ? compound_sbt_offsets[cid]
                : 10u;
            oi.visibilityMask = visible ? 0xFFu : 0u;
            oi.flags = 4u; // OPTIX_INSTANCE_FLAG_DISABLE_ANYHIT
            oi.traversableHandle = (compound_gas_handles != nullptr && cid < compound_count)
                ? compound_gas_handles[cid]
                : 0ull;
            oi.pad[0] = 0u;
            oi.pad[1] = 0u;
            dst_optix_instances[oi_idx] = oi;
        }
    }
}
#endif
"#;

/// Grow-only typed GPU byte buffer for zero steady-state VRAM reallocations.
struct GrowCudaBuf<T: Pod> {
    buf: Option<DeviceBuffer<u8>>,
    cap_bytes: usize,
    len_elems: usize,
    _marker: PhantomData<T>,
}

impl<T: Pod> Default for GrowCudaBuf<T> {
    fn default() -> Self {
        Self {
            buf: None,
            cap_bytes: 0,
            len_elems: 0,
            _marker: PhantomData,
        }
    }
}

impl<T: Pod> GrowCudaBuf<T> {
    #[inline]
    fn device_ptr(&self) -> u64 {
        if self.len_elems == 0 {
            0
        } else {
            self.buf.as_ref().map_or(0, DeviceBuffer::cu_deviceptr)
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.len_elems
    }

    fn ensure_len(&mut self, stream: &Stream, count: usize) -> Result<(), String> {
        let bytes = count.saturating_mul(std::mem::size_of::<T>());
        if bytes > self.cap_bytes {
            let alloc_bytes = bytes.max(256).next_power_of_two();
            let new_buf = DeviceBuffer::<u8>::zeroed(stream, alloc_bytes)
                .map_err(|e| format!("CUDA crowd buffer alloc ({alloc_bytes} B) failed: {e}"))?;
            self.buf = Some(new_buf);
            self.cap_bytes = alloc_bytes;
        }
        self.len_elems = count;
        Ok(())
    }

    fn upload(&mut self, stream: &Stream, data: &[T]) -> Result<(), String> {
        self.ensure_len(stream, data.len())?;
        if data.is_empty() {
            return Ok(());
        }
        let raw: &[u8] = bytemuck::cast_slice(data);
        let ptr = self.device_ptr();
        // SAFETY: `self.buf` has capacity >= `raw.len()` bytes on `stream`; `T: Pod` guarantees
        // contiguous, initialized bytes and `cuMemcpyHtoDAsync_v2` reads `raw.len()` bytes.
        let res = unsafe {
            cu::cuMemcpyHtoDAsync_v2(ptr, raw.as_ptr().cast(), raw.len(), stream.cu_stream())
        };
        if res != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!("CUDA crowd H2D copy failed ({res:?})"));
        }
        Ok(())
    }

    fn download_into(&self, stream: &Stream, dst: &mut [T]) -> Result<(), String> {
        let count = dst.len().min(self.len_elems);
        if count == 0 {
            return Ok(());
        }
        let raw: &mut [u8] = bytemuck::cast_slice_mut(&mut dst[..count]);
        let ptr = self.device_ptr();
        // SAFETY: `self.buf` has at least `raw.len()` valid bytes on `stream`; `dst` is writable
        // and caller synchronizes `stream` before reading `dst` (or uses synchronous D2H).
        let res = unsafe {
            cu::cuMemcpyDtoHAsync_v2(raw.as_mut_ptr().cast(), ptr, raw.len(), stream.cu_stream())
        };
        if res != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!("CUDA crowd D2H copy failed ({res:?})"));
        }
        Ok(())
    }
}

/// Device pointer targets for zero-copy GPU-direct crowd scatter into CSG + OptiX IAS buffers.
#[derive(Clone, Copy, Debug)]
pub struct GpuScatterTarget {
    /// Device pointer to `CsgGpuInstance` (`instances_buf`).
    pub csg_instances_ptr: u64,
    /// Device pointer to `[f32; 12]` previous-frame transforms (`prev_xforms_buf`).
    pub csg_prev_xforms_ptr: u64,
    /// Valid element count of `csg_instances_ptr` (and `csg_prev_xforms_ptr`).
    pub max_csg_instances: u32,
    /// Device pointer to `OptixInstance` array (`d_optix_instances`).
    pub optix_instances_ptr: u64,
    /// Leading classic instance offset (`rig.ias_classic_count`) in `d_optix_instances`.
    pub optix_slot_offset: u32,
    /// Total valid element count of `d_optix_instances`.
    pub max_optix_instances: u32,
}

/// Summary returned from a single GPU crowd step.
#[derive(Clone, Copy, Debug, Default)]
pub struct CudaCrowdStepReport {
    /// Host wall time for the GPU crowd step (ms).
    pub step_ms: f32,
    /// True when cumulative actor displacement exceeded [`CUDA_IAS_REBUILD_DISP_M`], requesting an
    /// amortized full OptiX IAS rebuild instead of an incremental `OPTIX_BUILD_OPERATION_UPDATE`.
    pub needs_full_ias_rebuild: bool,
}

/// Persistent CUDA/NVRTC articulated crowd simulator with grow-only VRAM buffers.
pub struct CudaCrowdBackend {
    cuda: Arc<CudaContext>,
    module: cu::CUmodule,
    prepare_fn: cu::CUfunction,
    step_fn: cu::CUfunction,
    scatter_fn: cu::CUfunction,
    disp_fn: cu::CUfunction,
    d_uniforms: GrowCudaBuf<SimUniformsPod>,
    d_archetypes: GrowCudaBuf<ArchetypeMetaPod>,
    d_descs: GrowCudaBuf<ActorDescPod>,
    d_grounds: GrowCudaBuf<GroundSamplePod>,
    d_intents: GrowCudaBuf<IntentPod>,
    d_states: GrowCudaBuf<ActorStatePod>,
    d_bones: GrowCudaBuf<BoneDescPod>,
    d_bone_outs: GrowCudaBuf<BoneOutPod>,
    d_prev_xforms: GrowCudaBuf<[f32; 12]>,
    d_world_joints: GrowCudaBuf<[f32; 4]>,
    d_bone_slot_map: GrowCudaBuf<u32>,
    d_gas_handles: GrowCudaBuf<u64>,
    d_sbt_offsets: GrowCudaBuf<u32>,
    d_dem_uniforms: GrowCudaBuf<DemUniformsPod>,
    d_dem_heights: GrowCudaBuf<f32>,
    d_walkway_pts: GrowCudaBuf<[f32; 2]>,
    d_walkway_spans: GrowCudaBuf<[u32; 2]>,
    d_rebuild_pos: GrowCudaBuf<[f32; 4]>,
    d_disp_flag: GrowCudaBuf<u32>,
    pinned_disp_flag: PinnedU32,
    has_pending_disp_flag: bool,
    uploaded_walkway_count: u32,
    host_archetypes: Vec<ArchetypeMetaPod>,
    host_walkway_pts: Vec<[f32; 2]>,
    host_walkway_spans: Vec<[u32; 2]>,
    host_rebuild_pos: Vec<[f32; 4]>,
    scene_dirty: bool,
    intents_dirty: bool,
    topology_dirty: bool,
    cpu_bones_stale: bool,
    uploaded_actor_count: usize,
    uploaded_bone_count: usize,
}

impl Drop for CudaCrowdBackend {
    fn drop(&mut self) {
        if !self.module.is_null() && self.cuda.bind_to_thread().is_ok() {
            // SAFETY: `self.module` was loaded by `cuModuleLoadData` on `self.cuda` and is
            // unloaded exactly once when `CudaCrowdBackend` drops.
            let rc = unsafe { cu::cuModuleUnload(self.module) };
            if rc != cu::cudaError_enum_CUDA_SUCCESS {
                eprintln!("nv-demo: CudaCrowdBackend cuModuleUnload failed ({rc:?})");
            }
            self.module = std::ptr::null_mut();
        }
    }
}

fn load_cuda_fn(module: cu::CUmodule, name: &str) -> Result<cu::CUfunction, String> {
    let cname = CString::new(name).map_err(|e| format!("CString {name} failed: {e}"))?;
    let mut func: cu::CUfunction = std::ptr::null_mut();
    // SAFETY: `module` is a valid loaded CUmodule on the bound context and `cname` is NUL-terminated.
    let rc = unsafe { cu::cuModuleGetFunction(&mut func, module, cname.as_ptr()) };
    if rc != cu::cudaError_enum_CUDA_SUCCESS || func.is_null() {
        return Err(format!("cuModuleGetFunction({name}) failed ({rc:?})"));
    }
    Ok(func)
}

impl CudaCrowdBackend {
    /// Compile `csg_actors::ACTOR_SIM_CUH` + GPU steering/DEM/scatter/displacement kernels via
    /// NVRTC (backed by the content-addressed disk PTX cache) and load the CUDA module.
    pub fn new(cuda: &Arc<CudaContext>) -> Result<Self, String> {
        cuda.bind_to_thread()
            .map_err(|e| format!("CUDA bind failed for CudaCrowdBackend: {e}"))?;

        let mut full_src = String::with_capacity(ACTOR_SIM_CUH.len() + SCATTER_KERNEL_CU.len() + 2);
        full_src.push_str(ACTOR_SIM_CUH);
        full_src.push('\n');
        full_src.push_str(SCATTER_KERNEL_CU);

        let ptx = crate::gpu_export::compile_ptx_cached(
            &full_src,
            "actor_sim_crowd.cu",
            "compute_75",
            &[],
            &[],
        )?;

        let mut module: cu::CUmodule = std::ptr::null_mut();
        // SAFETY: `ptx` is a NUL-terminated valid PTX byte buffer produced by NVRTC and `cuda`
        // is current on the calling thread.
        let load_res = unsafe { cu::cuModuleLoadData(&mut module, ptx.as_ptr().cast()) };
        if load_res != cu::cudaError_enum_CUDA_SUCCESS || module.is_null() {
            return Err(format!("cuModuleLoadData(actor_sim) failed ({load_res:?})"));
        }

        let load_all = || -> Result<(cu::CUfunction, cu::CUfunction, cu::CUfunction, cu::CUfunction), String> {
            Ok((
                load_cuda_fn(module, "prepare_crowd_steering_and_grounds")?,
                load_cuda_fn(module, "step_and_emit_actors_cuda")?,
                load_cuda_fn(module, "scatter_crowd_to_csg_and_optix")?,
                load_cuda_fn(module, "check_crowd_ias_displacement")?,
            ))
        };
        let (prepare_fn, step_fn, scatter_fn, disp_fn) = match load_all() {
            Ok(fns) => fns,
            Err(err) => {
                // SAFETY: unloading valid `module` on error path prevents leak.
                let rc = unsafe { cu::cuModuleUnload(module) };
                if rc != cu::cudaError_enum_CUDA_SUCCESS {
                    eprintln!("nv-demo: cuModuleUnload on actor_sim error path failed ({rc:?})");
                }
                return Err(err);
            }
        };

        let pinned_disp_flag = match PinnedU32::new(cuda) {
            Ok(p) => p,
            Err(err) => {
                // SAFETY: unloading valid `module` on error path prevents leak.
                let rc = unsafe { cu::cuModuleUnload(module) };
                if rc != cu::cudaError_enum_CUDA_SUCCESS {
                    eprintln!("nv-demo: cuModuleUnload on pinned flag error failed ({rc:?})");
                }
                return Err(err);
            }
        };

        Ok(Self {
            cuda: cuda.clone(),
            module,
            prepare_fn,
            step_fn,
            scatter_fn,
            disp_fn,
            d_uniforms: GrowCudaBuf::default(),
            d_archetypes: GrowCudaBuf::default(),
            d_descs: GrowCudaBuf::default(),
            d_grounds: GrowCudaBuf::default(),
            d_intents: GrowCudaBuf::default(),
            d_states: GrowCudaBuf::default(),
            d_bones: GrowCudaBuf::default(),
            d_bone_outs: GrowCudaBuf::default(),
            d_prev_xforms: GrowCudaBuf::default(),
            d_world_joints: GrowCudaBuf::default(),
            d_bone_slot_map: GrowCudaBuf::default(),
            d_gas_handles: GrowCudaBuf::default(),
            d_sbt_offsets: GrowCudaBuf::default(),
            d_dem_uniforms: GrowCudaBuf::default(),
            d_dem_heights: GrowCudaBuf::default(),
            d_walkway_pts: GrowCudaBuf::default(),
            d_walkway_spans: GrowCudaBuf::default(),
            d_rebuild_pos: GrowCudaBuf::default(),
            d_disp_flag: GrowCudaBuf::default(),
            pinned_disp_flag,
            has_pending_disp_flag: false,
            uploaded_walkway_count: 0,
            host_archetypes: Vec::with_capacity(8),
            host_walkway_pts: Vec::with_capacity(256),
            host_walkway_spans: Vec::with_capacity(64),
            host_rebuild_pos: Vec::with_capacity(64),
            scene_dirty: true,
            intents_dirty: true,
            topology_dirty: true,
            cpu_bones_stale: false,
            uploaded_actor_count: 0,
            uploaded_bone_count: 0,
        })
    }

    /// Mark all crowd buffers dirty (call after scene rebuild or `spawn_crowd_stress`).
    #[inline]
    pub fn mark_scene_dirty(&mut self) {
        self.scene_dirty = true;
        self.intents_dirty = true;
        self.topology_dirty = true;
        self.cpu_bones_stale = false;
        self.has_pending_disp_flag = false;
    }

    /// Mark `IntentPod` buffer dirty (call after `poke_crowd` or `freeze_pose` toggle).
    #[inline]
    pub fn mark_intents_dirty(&mut self) {
        self.intents_dirty = true;
    }

    /// Mark GPU scatter mapping dirty (call after `CsgSyncResult::FullRepack`).
    #[inline]
    pub fn mark_topology_dirty(&mut self) {
        self.topology_dirty = true;
    }

    /// Returns `true` if GPU bone outputs have advanced ahead of CPU `CrowdSim::bone_outs`.
    #[inline]
    pub fn cpu_bones_stale(&self) -> bool {
        self.cpu_bones_stale
    }

    /// Upload updated `bone_slot_map`, per-compound GAS handles, and per-compound SBT offsets.
    pub fn upload_scatter_topology(
        &mut self,
        stream: &Stream,
        bone_slot_map: &[u32],
        gas_handles: &[u64],
        sbt_offsets: &[u32],
    ) -> Result<(), String> {
        self.d_bone_slot_map.upload(stream, bone_slot_map)?;
        self.d_gas_handles.upload(stream, gas_handles)?;
        self.d_sbt_offsets.upload(stream, sbt_offsets)?;
        self.topology_dirty = false;
        Ok(())
    }

    /// Returns `true` if scatter topology buffers need to be refreshed before scattering.
    #[inline]
    pub fn topology_dirty(&self) -> bool {
        self.topology_dirty
    }

    /// Synchronize GPU `states`, `intents`, `bone_outs`, `prev_xforms`, and `world_joints` back
    /// into `scene.crowd` (and optionally into `scene.world` dynamic instances) on demand right
    /// before camera repacks, `PokeCrowd`, `Ctrl+Click` picking, or SDG JSON export.
    pub fn sync_cpu_snapshot(
        &mut self,
        stream: &Stream,
        scene: &mut SceneBuilt,
        sync_to_world: bool,
    ) -> Result<(), String> {
        if !self.cpu_bones_stale || scene.crowd.actor_count() == 0 {
            return Ok(());
        }
        let n_actors = scene.crowd.actor_count();
        let n_bones = scene.crowd.bone_count();
        if self.d_states.len() >= n_actors && self.d_bone_outs.len() >= n_bones {
            self.d_states.download_into(stream, &mut scene.crowd.states)?;
            if self.d_intents.len() >= n_actors {
                self.d_intents
                    .download_into(stream, &mut scene.crowd.intents)?;
            }
            self.d_bone_outs
                .download_into(stream, &mut scene.crowd.bone_outs)?;
            self.d_prev_xforms
                .download_into(stream, &mut scene.crowd.prev_xforms)?;
            self.d_world_joints
                .download_into(stream, &mut scene.crowd.world_joints)?;
            stream
                .synchronize()
                .map_err(|e| format!("CUDA crowd CPU snapshot sync failed: {e}"))?;
            if sync_to_world {
                scene.sync_crowd_to_world();
            }
        }
        self.cpu_bones_stale = false;
        Ok(())
    }

    /// Step `scene.crowd` entirely on GPU (DEM ground sampling + walkway patrol steering +
    /// LIPM/PD/FK/IK + scatter into `CsgGpuInstance`/`prev_xforms`/`OptixInstance` + 1-word async
    /// displacement flag) with **zero mid-frame host synchronization** in steady state.
    #[expect(
        clippy::too_many_arguments,
        reason = "coordinates CUDA stream, scene state, timestep, camera eye, scatter targets, and readback policy cleanly"
    )]
    pub fn step_gpu_direct(
        &mut self,
        stream: &Stream,
        scene: &mut SceneBuilt,
        dt: f32,
        time_s: f32,
        cam_eye: [f32; 3],
        scatter: Option<GpuScatterTarget>,
        sync_world_joints: bool,
    ) -> Result<CudaCrowdStepReport, String> {
        let n_actors = scene.crowd.actor_count();
        let n_bones = scene.crowd.bone_count();
        if n_actors == 0 || n_bones == 0 {
            return Ok(CudaCrowdStepReport::default());
        }
        let t0 = Instant::now();

        let has_walkways = !scene.hints.walkways.is_empty();
        let ground_offset = if has_walkways {
            CROWD_SIDEWALK_CLEARANCE_M
        } else {
            0.0
        };

        // 1. Upload static/cold scene & DEM/walkway buffers only when scene or crowd size changed.
        let size_changed =
            n_actors != self.uploaded_actor_count || n_bones != self.uploaded_bone_count;
        if self.scene_dirty || size_changed {
            scene.crowd.pack_archetype_metas_into(&mut self.host_archetypes);
            self.d_archetypes.upload(stream, &self.host_archetypes)?;
            self.d_descs.upload(stream, &scene.crowd.descs)?;
            self.d_bones.upload(stream, &scene.crowd.bones)?;
            self.d_states.upload(stream, &scene.crowd.states)?;
            self.d_bone_outs.upload(stream, &scene.crowd.bone_outs)?;
            self.d_prev_xforms.upload(stream, &scene.crowd.prev_xforms)?;
            self.d_world_joints.upload(stream, &scene.crowd.world_joints)?;
            self.d_grounds.ensure_len(stream, n_actors)?;
            self.d_disp_flag.ensure_len(stream, 1)?;

            // Upload DEM elevation grid (or flat constant) once per scene.
            let dem_u = if let Some(grid) = scene.hints.elevation.as_grid()
                && grid.is_valid()
            {
                self.d_dem_heights.upload(stream, &grid.heights)?;
                DemUniformsPod {
                    origin_x: grid.origin_xz[0],
                    origin_z: grid.origin_xz[1],
                    cell_x: grid.cell_xz[0].abs(),
                    cell_z: grid.cell_xz[1].abs(),
                    nx: grid.nx,
                    nz: grid.nz,
                    flat_y: 0.0,
                    offset_m: ground_offset,
                }
            } else {
                self.d_dem_heights.ensure_len(stream, 0)?;
                DemUniformsPod {
                    origin_x: 0.0,
                    origin_z: 0.0,
                    cell_x: 1.0,
                    cell_z: 1.0,
                    nx: 0,
                    nz: 0,
                    flat_y: scene.hints.elevation.sample(0.0, 0.0),
                    offset_m: ground_offset,
                }
            };
            self.d_dem_uniforms.upload(stream, &[dem_u])?;

            // Flatten and upload walkway patrol polylines once per scene.
            self.host_walkway_pts.clear();
            self.host_walkway_spans.clear();
            for w in &scene.hints.walkways {
                let start = u32::try_from(self.host_walkway_pts.len()).unwrap_or(u32::MAX);
                let len = u32::try_from(w.pts.len()).unwrap_or(0);
                self.host_walkway_spans.push([start, len]);
                for p in &w.pts {
                    self.host_walkway_pts.push([p.x, p.z]);
                }
            }
            self.d_walkway_pts.upload(stream, &self.host_walkway_pts)?;
            self.d_walkway_spans
                .upload(stream, &self.host_walkway_spans)?;
            self.uploaded_walkway_count = self.host_walkway_spans.len() as u32;

            // Upload initial rebuild-anchor positions for amortized IAS rebuild checks.
            self.host_rebuild_pos.clear();
            self.host_rebuild_pos.reserve(n_actors);
            for st in &scene.crowd.states {
                self.host_rebuild_pos
                    .push([st.pos_yaw[0], st.pos_yaw[1], st.pos_yaw[2], 0.0]);
            }
            self.d_rebuild_pos.upload(stream, &self.host_rebuild_pos)?;
            self.pinned_disp_flag.write(0);
            self.has_pending_disp_flag = false;

            self.uploaded_actor_count = n_actors;
            self.uploaded_bone_count = n_bones;
            self.scene_dirty = false;
            self.intents_dirty = true;
        }

        // 2. Upload intents ONLY when externally modified (scene init, PokeCrowd, FreezePose).
        if self.intents_dirty {
            self.d_intents.upload(stream, &scene.crowd.intents)?;
            for intent in &mut scene.crowd.intents {
                let frozen = (intent.flags & intent_flags::FREEZE_POSE) != 0;
                if !frozen && (intent.flags & intent_flags::MASK_IMPULSE) != 0 {
                    intent.impulse_stance[0] = 0.0;
                    intent.impulse_stance[1] = 0.0;
                    intent.impulse_stance[2] = 0.0;
                    intent.flags &= !intent_flags::MASK_IMPULSE;
                }
            }
            self.intents_dirty = false;
        }

        // Read the 1-word async displacement flag populated by the previous frame (whose stream
        // already completed at the single end-of-frame fence — zero mid-frame sync!).
        let needs_full_ias_rebuild = if self.has_pending_disp_flag {
            self.pinned_disp_flag.read() != 0
        } else {
            false
        };
        if needs_full_ias_rebuild {
            self.pinned_disp_flag.write(0);
        }

        let block_x = 128u32;
        let grid_x = (n_actors as u32).div_ceil(block_x).max(1);

        // 3. GPU-side walkway patrol steering + DEM ground/foot sampling + zeroing `d_disp_flag`.
        {
            let mut actor_count_u32 = n_actors as u32;
            let mut quad_id = ArchetypeKind::Quadruped as u32;
            let mut p_states = self.d_states.device_ptr();
            let mut p_descs = self.d_descs.device_ptr();
            let mut p_intents = self.d_intents.device_ptr();
            let mut p_grounds = self.d_grounds.device_ptr();
            let mut p_dem_u = self.d_dem_uniforms.device_ptr();
            let mut p_dem_h = self.d_dem_heights.device_ptr();
            let mut p_wpts = self.d_walkway_pts.device_ptr();
            let mut p_wspans = self.d_walkway_spans.device_ptr();
            let mut num_w = self.uploaded_walkway_count;
            let mut p_flag = self.d_disp_flag.device_ptr();
            let mut prep_args: [*mut c_void; 12] = [
                (&raw mut actor_count_u32).cast(),
                (&raw mut quad_id).cast(),
                (&raw mut p_states).cast(),
                (&raw mut p_descs).cast(),
                (&raw mut p_intents).cast(),
                (&raw mut p_grounds).cast(),
                (&raw mut p_dem_u).cast(),
                (&raw mut p_dem_h).cast(),
                (&raw mut p_wpts).cast(),
                (&raw mut p_wspans).cast(),
                (&raw mut num_w).cast(),
                (&raw mut p_flag).cast(),
            ];
            // SAFETY: `self.prepare_fn` matches the 12-parameter ABI of
            // `prepare_crowd_steering_and_grounds`; all non-null device buffers are sized for
            // `n_actors` / uploaded scene tables and stay live on `stream`.
            let prep_res = unsafe {
                cu::cuLaunchKernel(
                    self.prepare_fn,
                    grid_x,
                    1,
                    1,
                    block_x,
                    1,
                    1,
                    0,
                    stream.cu_stream(),
                    prep_args.as_mut_ptr(),
                    std::ptr::null_mut(),
                )
            };
            if prep_res != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!(
                    "cuLaunchKernel(prepare_crowd_steering_and_grounds) failed ({prep_res:?})"
                ));
            }
        }

        let u_sim = SimUniformsPod {
            dt: dt.clamp(1e-4, 1.0 / 30.0),
            time_s,
            gravity: 9.81,
            cam_x: cam_eye[0],
            cam_y: cam_eye[1],
            cam_z: cam_eye[2],
            actor_count: n_actors as u32,
            total_bones: n_bones as u32,
        };
        self.d_uniforms.upload(stream, &[u_sim])?;

        // 4. Launch `step_and_emit_actors_cuda` on `stream`.
        let mut p_u_sim = self.d_uniforms.device_ptr();
        let mut p_arch = self.d_archetypes.device_ptr();
        let mut p_descs = self.d_descs.device_ptr();
        let mut p_grounds = self.d_grounds.device_ptr();
        let mut p_intents = self.d_intents.device_ptr();
        let mut p_states = self.d_states.device_ptr();
        let mut p_bones = self.d_bones.device_ptr();
        let mut p_bone_outs = self.d_bone_outs.device_ptr();
        let mut p_prev_xforms = self.d_prev_xforms.device_ptr();
        let mut p_world_joints = self.d_world_joints.device_ptr();
        let mut p_optix_null: u64 = 0;
        let mut p_slot_map_null: u64 = 0;
        let mut p_gas_null: u64 = 0;
        let mut p_sbt_null: u64 = 0;
        let mut optix_off_zero: u32 = 0;
        let mut lod1 = scene.crowd.lod1_dist_m;
        let mut lod2 = scene.crowd.lod2_dist_m;
        let mut cull = scene.crowd.cull_dist_m;

        let mut step_args: [*mut c_void; 18] = [
            (&raw mut p_u_sim).cast(),
            (&raw mut p_arch).cast(),
            (&raw mut p_descs).cast(),
            (&raw mut p_grounds).cast(),
            (&raw mut p_intents).cast(),
            (&raw mut p_states).cast(),
            (&raw mut p_bones).cast(),
            (&raw mut p_bone_outs).cast(),
            (&raw mut p_prev_xforms).cast(),
            (&raw mut p_world_joints).cast(),
            (&raw mut p_optix_null).cast(),
            (&raw mut p_slot_map_null).cast(),
            (&raw mut p_gas_null).cast(),
            (&raw mut p_sbt_null).cast(),
            (&raw mut optix_off_zero).cast(),
            (&raw mut lod1).cast(),
            (&raw mut lod2).cast(),
            (&raw mut cull).cast(),
        ];

        // SAFETY: `self.step_fn` matches the 18-parameter ABI of `step_and_emit_actors_cuda`;
        // every non-null device pointer has capacity for at least `n_actors`/`n_bones` elements
        // and outlives `stream` synchronization.
        let step_res = unsafe {
            cu::cuLaunchKernel(
                self.step_fn,
                grid_x,
                1,
                1,
                block_x,
                1,
                1,
                0,
                stream.cu_stream(),
                step_args.as_mut_ptr(),
                std::ptr::null_mut(),
            )
        };
        if step_res != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!(
                "cuLaunchKernel(step_and_emit_actors_cuda) failed ({step_res:?})"
            ));
        }

        // 5. Scatter GPU-directly into `csg_instances`, `csg_prev_xforms`, and `d_optix_instances`.
        if let Some(target) = scatter
            && target.csg_instances_ptr != 0
            && self.d_bone_slot_map.len() >= n_bones
        {
            let mut p_src_bones = self.d_bone_outs.device_ptr();
            let mut p_src_prev = self.d_prev_xforms.device_ptr();
            let mut p_slot_map = self.d_bone_slot_map.device_ptr();
            let mut total_bones_u32 = n_bones as u32;
            let mut max_csg = target.max_csg_instances;
            let mut extra_flags = SCATTER_VISIBLE_EXTRA_FLAGS;
            let mut p_dst_csg = target.csg_instances_ptr;
            let mut p_dst_prev = target.csg_prev_xforms_ptr;
            let mut p_dst_optix = target.optix_instances_ptr;
            let mut optix_off = target.optix_slot_offset;
            let mut max_optix = target.max_optix_instances;
            let mut p_gas = self.d_gas_handles.device_ptr();
            let mut p_sbt = self.d_sbt_offsets.device_ptr();
            let mut compound_count = self.d_gas_handles.len().min(self.d_sbt_offsets.len()) as u32;

            let mut scat_args: [*mut c_void; 14] = [
                (&raw mut p_src_bones).cast(),
                (&raw mut p_src_prev).cast(),
                (&raw mut p_slot_map).cast(),
                (&raw mut total_bones_u32).cast(),
                (&raw mut max_csg).cast(),
                (&raw mut extra_flags).cast(),
                (&raw mut p_dst_csg).cast(),
                (&raw mut p_dst_prev).cast(),
                (&raw mut p_dst_optix).cast(),
                (&raw mut optix_off).cast(),
                (&raw mut max_optix).cast(),
                (&raw mut p_gas).cast(),
                (&raw mut p_sbt).cast(),
                (&raw mut compound_count).cast(),
            ];
            let s_block = 256u32;
            let s_grid = total_bones_u32.div_ceil(s_block).max(1);
            // SAFETY: `self.scatter_fn` matches the 14-parameter ABI of
            // `scatter_crowd_to_csg_and_optix`; destination writes are strictly bounds-checked
            // against `max_csg_instances` and `max_optix_instances` on `stream`.
            let scat_res = unsafe {
                cu::cuLaunchKernel(
                    self.scatter_fn,
                    s_grid,
                    1,
                    1,
                    s_block,
                    1,
                    1,
                    0,
                    stream.cu_stream(),
                    scat_args.as_mut_ptr(),
                    std::ptr::null_mut(),
                )
            };
            if scat_res != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!(
                    "cuLaunchKernel(scatter_crowd_to_csg_and_optix) failed ({scat_res:?})"
                ));
            }
        }

        // 6. Async 1-word GPU displacement check + 4-byte page-locked D2H (ZERO mid-frame sync!).
        {
            let mut actor_count_u32 = n_actors as u32;
            let mut p_states_chk = self.d_states.device_ptr();
            let mut p_rebuild = self.d_rebuild_pos.device_ptr();
            let mut max_disp_sq = CUDA_IAS_REBUILD_DISP_M * CUDA_IAS_REBUILD_DISP_M;
            let mut reset_anchors = u32::from(needs_full_ias_rebuild);
            let mut p_flag = self.d_disp_flag.device_ptr();
            let mut disp_args: [*mut c_void; 6] = [
                (&raw mut actor_count_u32).cast(),
                (&raw mut p_states_chk).cast(),
                (&raw mut p_rebuild).cast(),
                (&raw mut max_disp_sq).cast(),
                (&raw mut reset_anchors).cast(),
                (&raw mut p_flag).cast(),
            ];
            // SAFETY: `self.disp_fn` matches the 6-parameter ABI of `check_crowd_ias_displacement`;
            // `d_states`, `d_rebuild_pos` hold `n_actors` items and `d_disp_flag` holds 1 `u32`.
            let disp_res = unsafe {
                cu::cuLaunchKernel(
                    self.disp_fn,
                    grid_x,
                    1,
                    1,
                    block_x,
                    1,
                    1,
                    0,
                    stream.cu_stream(),
                    disp_args.as_mut_ptr(),
                    std::ptr::null_mut(),
                )
            };
            if disp_res != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!(
                    "cuLaunchKernel(check_crowd_ias_displacement) failed ({disp_res:?})"
                ));
            }
            // SAFETY: `pinned_disp_flag` is 4-byte page-locked host memory and `d_disp_flag` is
            // 4-byte device memory on `stream`; the end-of-frame fence completes this DMA before
            // the next frame reads `pinned_disp_flag`.
            let d2h_flag = unsafe {
                cu::cuMemcpyDtoHAsync_v2(
                    self.pinned_disp_flag.as_mut_c_void(),
                    self.d_disp_flag.device_ptr(),
                    std::mem::size_of::<u32>(),
                    stream.cu_stream(),
                )
            };
            if d2h_flag != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!(
                    "CUDA crowd async displacement flag D2H failed ({d2h_flag:?})"
                ));
            }
            self.has_pending_disp_flag = true;
        }

        // Only synchronize mid-frame when actively possessing a crowd actor (camera needs live
        // joint transforms on CPU for head/shoulder mount placement).
        if sync_world_joints {
            self.d_states.download_into(stream, &mut scene.crowd.states)?;
            self.d_world_joints
                .download_into(stream, &mut scene.crowd.world_joints)?;
            stream
                .synchronize()
                .map_err(|e| format!("CUDA crowd possess joint sync failed: {e}"))?;
        }
        self.cpu_bones_stale = true;

        Ok(CudaCrowdStepReport {
            step_ms: t0.elapsed().as_secs_f32() * 1000.0,
            needs_full_ias_rebuild,
        })
    }

    /// Standalone GPU step directly on a [`csg_actors::CrowdSim`] with caller-supplied `GroundSamplePod`s
    /// (used by parity tests and standalone 2,000-actor GPU benchmarks).
    #[cfg(test)]
    #[expect(
        clippy::too_many_arguments,
        reason = "test helper mirrors GPU step parameters directly for parity and stress benchmarks"
    )]
    pub fn step_crowd_raw_for_test(
        &mut self,
        stream: &Stream,
        crowd: &mut csg_actors::CrowdSim,
        grounds: &[GroundSamplePod],
        dt: f32,
        time_s: f32,
        cam_eye: [f32; 3],
        readback_all: bool,
    ) -> Result<f32, String> {
        let n_actors = crowd.actor_count();
        let n_bones = crowd.bone_count();
        if n_actors == 0 || n_bones == 0 {
            return Ok(0.0);
        }
        let t0 = Instant::now();
        let size_changed =
            n_actors != self.uploaded_actor_count || n_bones != self.uploaded_bone_count;
        if self.scene_dirty || size_changed {
            crowd.pack_archetype_metas_into(&mut self.host_archetypes);
            self.d_archetypes.upload(stream, &self.host_archetypes)?;
            self.d_descs.upload(stream, &crowd.descs)?;
            self.d_bones.upload(stream, &crowd.bones)?;
            self.d_states.upload(stream, &crowd.states)?;
            self.d_bone_outs.upload(stream, &crowd.bone_outs)?;
            self.d_prev_xforms.upload(stream, &crowd.prev_xforms)?;
            self.d_world_joints.upload(stream, &crowd.world_joints)?;
            self.uploaded_actor_count = n_actors;
            self.uploaded_bone_count = n_bones;
            self.scene_dirty = false;
            self.intents_dirty = true;
        }
        if self.intents_dirty {
            self.d_intents.upload(stream, &crowd.intents)?;
            for intent in &mut crowd.intents {
                let frozen = (intent.flags & intent_flags::FREEZE_POSE) != 0;
                if !frozen && (intent.flags & intent_flags::MASK_IMPULSE) != 0 {
                    intent.impulse_stance[0] = 0.0;
                    intent.impulse_stance[1] = 0.0;
                    intent.impulse_stance[2] = 0.0;
                    intent.flags &= !intent_flags::MASK_IMPULSE;
                }
            }
            self.intents_dirty = false;
        }
        self.d_grounds.upload(stream, grounds)?;
        let u_sim = SimUniformsPod {
            dt: dt.clamp(1e-4, 1.0 / 30.0),
            time_s,
            gravity: 9.81,
            cam_x: cam_eye[0],
            cam_y: cam_eye[1],
            cam_z: cam_eye[2],
            actor_count: n_actors as u32,
            total_bones: n_bones as u32,
        };
        self.d_uniforms.upload(stream, &[u_sim])?;

        let mut p_u_sim = self.d_uniforms.device_ptr();
        let mut p_arch = self.d_archetypes.device_ptr();
        let mut p_descs = self.d_descs.device_ptr();
        let mut p_grounds = self.d_grounds.device_ptr();
        let mut p_intents = self.d_intents.device_ptr();
        let mut p_states = self.d_states.device_ptr();
        let mut p_bones = self.d_bones.device_ptr();
        let mut p_bone_outs = self.d_bone_outs.device_ptr();
        let mut p_prev_xforms = self.d_prev_xforms.device_ptr();
        let mut p_world_joints = self.d_world_joints.device_ptr();
        let mut p_null: u64 = 0;
        let mut off_zero: u32 = 0;
        let mut lod1 = crowd.lod1_dist_m;
        let mut lod2 = crowd.lod2_dist_m;
        let mut cull = crowd.cull_dist_m;
        let mut step_args: [*mut c_void; 18] = [
            (&raw mut p_u_sim).cast(),
            (&raw mut p_arch).cast(),
            (&raw mut p_descs).cast(),
            (&raw mut p_grounds).cast(),
            (&raw mut p_intents).cast(),
            (&raw mut p_states).cast(),
            (&raw mut p_bones).cast(),
            (&raw mut p_bone_outs).cast(),
            (&raw mut p_prev_xforms).cast(),
            (&raw mut p_world_joints).cast(),
            (&raw mut p_null).cast(),
            (&raw mut p_null).cast(),
            (&raw mut p_null).cast(),
            (&raw mut p_null).cast(),
            (&raw mut off_zero).cast(),
            (&raw mut lod1).cast(),
            (&raw mut lod2).cast(),
            (&raw mut cull).cast(),
        ];
        let block_x = 128u32;
        let grid_x = (n_actors as u32).div_ceil(block_x).max(1);
        // SAFETY: `self.step_fn` matches the 18-parameter ABI of `step_and_emit_actors_cuda`
        // and all non-null device buffers are allocated on `stream`.
        let res = unsafe {
            cu::cuLaunchKernel(
                self.step_fn,
                grid_x,
                1,
                1,
                block_x,
                1,
                1,
                0,
                stream.cu_stream(),
                step_args.as_mut_ptr(),
                std::ptr::null_mut(),
            )
        };
        if res != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!("cuLaunchKernel failed ({res:?})"));
        }
        if readback_all {
            self.d_states.download_into(stream, &mut crowd.states)?;
            self.d_bone_outs.download_into(stream, &mut crowd.bone_outs)?;
            self.d_prev_xforms
                .download_into(stream, &mut crowd.prev_xforms)?;
            self.d_world_joints
                .download_into(stream, &mut crowd.world_joints)?;
        }
        stream
            .synchronize()
            .map_err(|e| format!("test stream sync failed: {e}"))?;
        Ok(t0.elapsed().as_secs_f32() * 1000.0)
    }
}

/// Unpack a walkway patrol handle matching `csg_scene::encode_walkway_target`.
#[cfg(test)]
#[inline]
const fn decode_walkway_target(target_id: u32) -> Option<(usize, usize, bool)> {
    if target_id == u32::MAX {
        None
    } else {
        let walkway_idx = ((target_id >> 16) & 0x7FFF) as usize;
        let wp_idx = ((target_id >> 1) & 0x7FFF) as usize;
        let reverse = (target_id & 1) != 0;
        Some((walkway_idx, wp_idx, reverse))
    }
}

/// Update walkway patrol steering intents in-place (zero heap allocations), matching
/// `SceneBuilt::step_crowd` verbatim (retained in tests for GPU parity checks).
#[cfg(test)]
fn update_walkway_intents(scene: &mut SceneBuilt) {
    let actor_count = scene.crowd.actor_count();
    for i in 0..actor_count {
        let intent = &mut scene.crowd.intents[i];
        if (intent.flags & intent_flags::FREEZE_POSE) != 0 {
            continue;
        }
        let Some((w_idx, mut wp_idx, mut reverse)) = decode_walkway_target(intent.target_id) else {
            continue;
        };
        let Some(walkway) = scene.hints.walkways.get(w_idx) else {
            continue;
        };
        if walkway.pts.len() < 2 {
            continue;
        }
        let last_idx = walkway.pts.len() - 1;
        wp_idx = wp_idx.min(last_idx);

        let st = scene.crowd.states[i];
        let pos_x = st.pos_yaw[0];
        let pos_z = st.pos_yaw[2];
        let mut wp = walkway.pts[wp_idx];
        let mut dx = wp.x - pos_x;
        let mut dz = wp.z - pos_z;
        if dx * dx + dz * dz < 0.75 * 0.75 {
            if !reverse {
                if wp_idx < last_idx {
                    wp_idx += 1;
                } else {
                    reverse = true;
                    wp_idx = last_idx.saturating_sub(1);
                }
            } else if wp_idx > 0 {
                wp_idx -= 1;
            } else {
                reverse = false;
                wp_idx = 1.min(last_idx);
            }
            wp = walkway.pts[wp_idx];
            dx = wp.x - pos_x;
            dz = wp.z - pos_z;
        }

        let dist = (dx * dx + dz * dz).sqrt();
        if dist.is_finite() && dist > 1e-3 {
            let speed = if intent.move_cmd[3].is_finite() && intent.move_cmd[3] > 0.1 {
                intent.move_cmd[3].clamp(0.45, 3.2)
            } else {
                1.25
            };
            let inv_d = 1.0 / dist;
            intent.move_cmd = [dx * inv_d * speed, dz * inv_d * speed, dx.atan2(dz), speed];
            intent.flags |= intent_flags::MASK_VEL | intent_flags::MASK_HEADING;
        }
        intent.target_id = encode_walkway_target(w_idx, wp_idx, reverse);
    }
}

/// Sample `GroundSamplePod`s for every actor in `scene.crowd` matching `CrowdSim::step_with_policy`
/// (retained in tests for GPU DEM parity checks).
#[cfg(test)]
fn sample_scene_grounds(scene: &SceneBuilt, offset_m: f32, out: &mut Vec<GroundSamplePod>) {
    let n = scene.crowd.actor_count();
    out.clear();
    out.reserve(n);
    let elev = &scene.hints.elevation;
    for i in 0..n {
        let st = scene.crowd.states[i];
        let desc = scene.crowd.descs[i];
        let hy = elev.sample(st.pos_yaw[0], st.pos_yaw[2]) + offset_m;
        let nrm = elev.sample_normal(st.pos_yaw[0], st.pos_yaw[2]);
        let yaw = st.pos_yaw[3];
        let (sy, cy) = yaw.sin_cos();
        let hs = desc.scale_dims[0].max(0.25);
        let is_quad = desc.archetype_id == ArchetypeKind::Quadruped as u32;
        let foot_offsets: [(f32, f32); 4] = if is_quad {
            [
                (-0.11 * cy + 0.32 * hs * sy, 0.11 * sy + 0.32 * hs * cy),
                (0.11 * cy + 0.32 * hs * sy, -0.11 * sy + 0.32 * hs * cy),
                (-0.11 * cy - 0.26 * hs * sy, 0.11 * sy - 0.26 * hs * cy),
                (0.11 * cy - 0.26 * hs * sy, -0.11 * sy - 0.26 * hs * cy),
            ]
        } else {
            [
                (-0.22 * cy, 0.22 * sy),
                (0.22 * cy, -0.22 * sy),
                (-0.11 * cy, 0.11 * sy),
                (0.11 * cy, -0.11 * sy),
            ]
        };
        let mut fh = [hy; 4];
        for (k, (ox, oz)) in foot_offsets.iter().enumerate() {
            fh[k] = elev.sample(st.pos_yaw[0] + ox, st.pos_yaw[2] + oz) + offset_m;
        }
        out.push(GroundSamplePod {
            height_normal: [hy, nrm.x, nrm.y, nrm.z],
            foot_heights: fh,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use csg_actors::{ActorSpawnParams, CrowdSim, FlatGround, SkinStyle, Vec3 as ActorVec3};
    use cuda_core::Context as CudaContext;

    fn make_4_archetype_crowd() -> CrowdSim {
        let mut crowd = CrowdSim::with_default_topologies();
        let archetypes = [
            ArchetypeKind::HumanHero,
            ArchetypeKind::HumanCrowd,
            ArchetypeKind::RoadSignStickman,
            ArchetypeKind::Quadruped,
        ];
        for (idx, arch) in archetypes.into_iter().enumerate() {
            let x = (idx as f32 - 1.5) * 2.5;
            let params = ActorSpawnParams::new(
                arch,
                ActorVec3::new(x, 0.0, 0.0),
                0.25 * idx as f32,
                500_000 + idx as u32,
            )
            .with_phase0(0.15 * idx as f32);
            let aidx = crowd.spawn_actor(params, |j| (j.role as u32 % 11, u32::MAX));
            crowd.intents[aidx].move_cmd = [0.6, 1.1, 0.5, 1.25];
            crowd.intents[aidx].flags |= intent_flags::MASK_VEL | intent_flags::MASK_HEADING;
        }
        // Apply a lateral poke impulse to actor 1 to exercise LIPM/PD stumble recovery parity.
        assert!(crowd.poke_actor(1, ActorVec3::new(1.8, 0.2, -0.6)));
        crowd
    }

    #[test]
    fn cuda_vs_cpu_crowd_step_parity_within_1e_3() {
        let ctx = CudaContext::new(0).expect("CUDA context on device 0");
        ctx.bind_to_thread().expect("bind CUDA context");
        let stream = ctx.default_stream();
        let mut backend = CudaCrowdBackend::new(&ctx).expect("NVRTC compile actor_sim.cuh");

        let mut cpu = make_4_archetype_crowd();
        let mut gpu = cpu.clone();
        let ground = FlatGround { height: 0.0 };
        let grounds = vec![
            GroundSamplePod {
                height_normal: [0.0, 0.0, 1.0, 0.0],
                foot_heights: [0.0; 4],
            };
            cpu.actor_count()
        ];
        let cam_eye = [0.0f32, 2.0, -8.0];
        let cam_vec = ActorVec3::from_array(cam_eye);
        let dt = 1.0 / 60.0;

        for step in 0..5 {
            let t = step as f32 * dt;
            cpu.step_analytical(&ground, cam_vec, dt, t);
            backend
                .step_crowd_raw_for_test(&stream, &mut gpu, &grounds, dt, t, cam_eye, true)
                .expect("GPU step");
        }

        let mut max_state_err = 0.0f32;
        for (c, g) in cpu.states.iter().zip(&gpu.states) {
            for (a, b) in c
                .pos_yaw
                .iter()
                .chain(&c.vel_yaw_rate)
                .chain(&c.gait_susp)
                .chain(&c.bal_angle_vel)
                .zip(
                    g.pos_yaw
                        .iter()
                        .chain(&g.vel_yaw_rate)
                        .chain(&g.gait_susp)
                        .chain(&g.bal_angle_vel),
                )
            {
                let err = (a - b).abs();
                assert!(err.is_finite(), "non-finite state diff");
                max_state_err = max_state_err.max(err);
            }
        }

        let mut max_xform_err = 0.0f32;
        for (c, g) in cpu.bone_outs.iter().zip(&gpu.bone_outs) {
            assert_eq!(c.compound_id, g.compound_id);
            assert_eq!(c.user_id, g.user_id);
            assert_eq!(c.flags, g.flags);
            for (a, b) in c.xform.iter().zip(&g.xform) {
                let err = (a - b).abs();
                assert!(err.is_finite(), "non-finite xform diff");
                max_xform_err = max_xform_err.max(err);
            }
        }

        let mut max_joint_err = 0.0f32;
        for (c, g) in cpu.world_joints.iter().zip(&gpu.world_joints) {
            for (a, b) in c.iter().zip(g) {
                let err = (a - b).abs();
                assert!(err.is_finite(), "non-finite joint diff");
                max_joint_err = max_joint_err.max(err);
            }
        }

        assert!(
            max_state_err < 1e-3,
            "CUDA vs CPU ActorStatePod parity exceeded 1e-3: max_err={max_state_err:.6}"
        );
        assert!(
            max_xform_err < 1e-3,
            "CUDA vs CPU BoneOutPod xform parity exceeded 1e-3: max_err={max_xform_err:.6}"
        );
        assert!(
            max_joint_err < 1e-3,
            "CUDA vs CPU world_joints parity exceeded 1e-3: max_err={max_joint_err:.6}"
        );
    }

    #[test]
    fn cuda_crowd_2000_actors_step_benchmark() {
        let ctx = CudaContext::new(0).expect("CUDA context on device 0");
        ctx.bind_to_thread().expect("bind CUDA context");
        let stream = ctx.default_stream();
        let mut backend = CudaCrowdBackend::new(&ctx).expect("NVRTC compile actor_sim.cuh");

        let mut crowd = CrowdSim::with_default_topologies();
        crowd.spawn_stress_grid(2000, ActorVec3::ZERO, 1.85, |_arch, _style: SkinStyle, j| {
            (j.role as u32 % 11, u32::MAX)
        });
        assert_eq!(crowd.actor_count(), 2000);
        let grounds = vec![
            GroundSamplePod {
                height_normal: [0.0, 0.0, 1.0, 0.0],
                foot_heights: [0.0; 4],
            };
            2000
        ];
        let cam_eye = [0.0f32, 12.0, -35.0];
        let dt = 1.0 / 60.0;

        // Warmup upload + first step.
        backend
            .step_crowd_raw_for_test(&stream, &mut crowd, &grounds, dt, 0.0, cam_eye, false)
            .expect("warmup GPU step");

        // Timed steady-state GPU steps (no full bone readback).
        let mut times_ms = Vec::with_capacity(10);
        for step in 1..=10 {
            let ms = backend
                .step_crowd_raw_for_test(
                    &stream,
                    &mut crowd,
                    &grounds,
                    dt,
                    step as f32 * dt,
                    cam_eye,
                    false,
                )
                .expect("steady GPU step");
            times_ms.push(ms);
        }
        // Final readback sanity check: every actor state & bone xform must be finite.
        backend
            .step_crowd_raw_for_test(&stream, &mut crowd, &grounds, dt, 11.0 * dt, cam_eye, true)
            .expect("readback GPU step");

        for st in &crowd.states {
            assert!(st.pos_yaw.iter().all(|v| v.is_finite()));
        }
        for b in &crowd.bone_outs {
            assert!(b.xform.iter().all(|v| v.is_finite()));
        }
        times_ms.sort_by(f32::total_cmp);
        let median_ms = times_ms[times_ms.len() / 2];
        assert!(
            median_ms < 25.0,
            "2,000-actor GPU step took unexpectedly long: {median_ms:.3} ms"
        );
        eprintln!(
            "cuda_crowd 2000 actors ({} bones): median {median_ms:.3} ms/step",
            crowd.bone_count()
        );
    }

    #[test]
    fn cuda_gpu_direct_dem_and_walkway_steering_matches_cpu_without_midframe_sync() {
        let ctx = CudaContext::new(0).expect("CUDA context on device 0");
        ctx.bind_to_thread().expect("bind CUDA context");
        let stream = ctx.default_stream();
        let mut backend = CudaCrowdBackend::new(&ctx).expect("NVRTC compile actor_sim.cuh");

        let reg = csg_scenes::default_scene_registry();
        let module = reg.get("gundo_osm").expect("gundo_osm registered");
        let mut gpu_scene = module.build(&csg_scene::SceneBuildConfig::default());
        if gpu_scene.crowd.actor_count() == 0 {
            return;
        }
        let mut cpu_scene = gpu_scene.clone();
        let mut cpu_grounds = Vec::new();
        let offset = if cpu_scene.hints.walkways.is_empty() {
            0.0
        } else {
            CROWD_SIDEWALK_CLEARANCE_M
        };
        update_walkway_intents(&mut cpu_scene);
        sample_scene_grounds(&cpu_scene, offset, &mut cpu_grounds);
        assert_eq!(cpu_grounds.len(), cpu_scene.crowd.actor_count());

        let cam_eye = [0.0f32, 10.0, 0.0];
        let dt = 1.0 / 60.0;
        for step in 0..4 {
            let t = step as f32 * dt;
            let _rep = backend
                .step_gpu_direct(&stream, &mut gpu_scene, dt, t, cam_eye, None, false)
                .expect("GPU direct step");
            // Simulate single end-of-frame fence.
            stream.synchronize().expect("end of frame fence");
        }
        backend
            .sync_cpu_snapshot(&stream, &mut gpu_scene, true)
            .expect("sync snapshot");
        for st in &gpu_scene.crowd.states {
            assert!(st.pos_yaw.iter().all(|v| v.is_finite()));
        }
        for b in &gpu_scene.crowd.bone_outs {
            assert!(b.xform.iter().all(|v| v.is_finite()));
        }
    }
}
