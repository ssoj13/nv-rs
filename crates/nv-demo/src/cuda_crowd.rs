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
use std::time::Instant;

use bytemuck::Pod;
use cuda_bindings as cu;
use cuda_core::{DeviceBuffer, Stream};

use csg_actors::{
    ACTOR_SIM_CUH, ActorDescPod, ActorStatePod, ArchetypeKind, ArchetypeMetaPod, BoneDescPod,
    BoneOutPod, GroundSamplePod, IntentPod, SimUniformsPod, intent_flags,
};
use csg_scene::{CROWD_SIDEWALK_CLEARANCE_M, SceneBuilt, encode_walkway_target, inst_flags};

/// Extra `CsgGpuInstance::flags` bits OR'd onto visible scattered crowd bone instances so shadows
/// and `Motion` AOV optical flow match CPU-packed dynamic actors identically.
const SCATTER_VISIBLE_EXTRA_FLAGS: u32 =
    inst_flags::DYNAMIC | inst_flags::CAST_SHADOW | inst_flags::MOTION_BLUR;

/// Maximum actor root displacement (in meters) before forcing a full OptiX IAS rebuild instead of
/// an incremental `OPTIX_BUILD_OPERATION_UPDATE` refit (preserves tight BVH traversal quality).
pub const CUDA_IAS_REBUILD_DISP_M: f32 = 8.0;

/// Bounds-checked GPU scatter kernel appended to `csg_actors::ACTOR_SIM_CUH` in the same NVRTC
/// compilation unit so partial frustum culling (`bone_slot_map[gidx] == 0xFFFFFFFFu`) is 100%
/// out-of-bounds-safe while writing directly into `csg_instances`, `csg_prev_xforms`, and
/// `d_optix_instances` in VRAM.
const SCATTER_KERNEL_CU: &str = r#"
#if defined(__CUDACC__) || defined(__CUDACC_RTC__)
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
                : 5u;
            oi.visibilityMask = visible ? 0xFFu : 0u;
            oi.flags = 0u;
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
    module: cu::CUmodule,
    step_fn: cu::CUfunction,
    scatter_fn: cu::CUfunction,
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
    host_archetypes: Vec<ArchetypeMetaPod>,
    host_grounds: Vec<GroundSamplePod>,
    last_rebuild_pos: Vec<[f32; 3]>,
    scene_dirty: bool,
    intents_dirty: bool,
    topology_dirty: bool,
    cpu_bones_stale: bool,
    uploaded_actor_count: usize,
    uploaded_bone_count: usize,
}

impl Drop for CudaCrowdBackend {
    fn drop(&mut self) {
        if !self.module.is_null() {
            // SAFETY: `self.module` was loaded by `cuModuleLoadData` and is unloaded exactly once
            // when `CudaCrowdBackend` drops.
            let _ = unsafe { cu::cuModuleUnload(self.module) };
            self.module = std::ptr::null_mut();
        }
    }
}

impl CudaCrowdBackend {
    /// Compile `csg_actors::ACTOR_SIM_CUH` + scatter kernel via NVRTC and load the CUDA module.
    pub fn new() -> Result<Self, String> {
        let mut full_src = String::with_capacity(ACTOR_SIM_CUH.len() + SCATTER_KERNEL_CU.len() + 2);
        full_src.push_str(ACTOR_SIM_CUH);
        full_src.push('\n');
        full_src.push_str(SCATTER_KERNEL_CU);

        let opts = nvrtc::CompileOptions::new()
            .arch("compute_75")
            .std("c++17")
            .fast_math();
        let mut prog = nvrtc::Program::new(&full_src, Some("actor_sim_crowd.cu"), &[])
            .map_err(|e| format!("NVRTC actor_sim program failed: {e}"))?;
        prog.compile(&opts)
            .map_err(|e| format!("NVRTC actor_sim compile failed: {e}"))?;
        let mut ptx = prog
            .ptx()
            .map_err(|e| format!("NVRTC actor_sim ptx failed: {e}"))?;
        if ptx.last() != Some(&0) {
            ptx.push(0);
        }

        let mut module: cu::CUmodule = std::ptr::null_mut();
        // SAFETY: `ptx` is a NUL-terminated valid PTX byte buffer produced by NVRTC and a CUDA
        // context is current on the calling thread.
        let load_res = unsafe { cu::cuModuleLoadData(&mut module, ptx.as_ptr().cast()) };
        if load_res != cu::cudaError_enum_CUDA_SUCCESS || module.is_null() {
            return Err(format!("cuModuleLoadData(actor_sim) failed ({load_res:?})"));
        }

        let step_name = CString::new("step_and_emit_actors_cuda")
            .map_err(|e| format!("CString step_fn failed: {e}"))?;
        let scatter_name = CString::new("scatter_crowd_to_csg_and_optix")
            .map_err(|e| format!("CString scatter_fn failed: {e}"))?;

        let mut step_fn: cu::CUfunction = std::ptr::null_mut();
        // SAFETY: `module` is valid and `step_name` is NUL-terminated.
        let res_step = unsafe { cu::cuModuleGetFunction(&mut step_fn, module, step_name.as_ptr()) };
        if res_step != cu::cudaError_enum_CUDA_SUCCESS || step_fn.is_null() {
            // SAFETY: unloading valid `module` on error path prevents leak.
            let _ = unsafe { cu::cuModuleUnload(module) };
            return Err(format!(
                "cuModuleGetFunction(step_and_emit_actors_cuda) failed ({res_step:?})"
            ));
        }

        let mut scatter_fn: cu::CUfunction = std::ptr::null_mut();
        // SAFETY: `module` is valid and `scatter_name` is NUL-terminated.
        let res_scat =
            unsafe { cu::cuModuleGetFunction(&mut scatter_fn, module, scatter_name.as_ptr()) };
        if res_scat != cu::cudaError_enum_CUDA_SUCCESS || scatter_fn.is_null() {
            // SAFETY: unloading valid `module` on error path prevents leak.
            let _ = unsafe { cu::cuModuleUnload(module) };
            return Err(format!(
                "cuModuleGetFunction(scatter_crowd_to_csg_and_optix) failed ({res_scat:?})"
            ));
        }

        Ok(Self {
            module,
            step_fn,
            scatter_fn,
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
            host_archetypes: Vec::with_capacity(8),
            host_grounds: Vec::with_capacity(64),
            last_rebuild_pos: Vec::with_capacity(64),
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

    /// Synchronize GPU `bone_outs`, `prev_xforms`, `world_joints`, and `states` back into
    /// `scene.crowd` (and optionally into `scene.world` dynamic instances) on demand right before
    /// camera repacks, `Ctrl+Click` picking, or SDG JSON export.
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

    /// Step `scene.crowd` on GPU and scatter directly into `CsgGpuInstance`, `prev_xforms`, and
    /// `OptixInstance` VRAM buffers when `scatter` is supplied.
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

        // 1. Update walkway patrol steering intents in place (matching `SceneBuilt::step_crowd`).
        let has_walkways = !scene.hints.walkways.is_empty();
        if has_walkways {
            update_walkway_intents(scene);
        }

        // 2. Sample ground height + normal + 4 nominal foot heights matching `CrowdSim`.
        let ground_offset = if has_walkways {
            CROWD_SIDEWALK_CLEARANCE_M
        } else {
            0.0
        };
        sample_scene_grounds(scene, ground_offset, &mut self.host_grounds);

        // 3. Upload static/cold buffers only when scene or crowd size changed.
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
            self.last_rebuild_pos.clear();
            self.last_rebuild_pos.reserve(n_actors);
            for st in &scene.crowd.states {
                self.last_rebuild_pos
                    .push([st.pos_yaw[0], st.pos_yaw[1], st.pos_yaw[2]]);
            }
            self.uploaded_actor_count = n_actors;
            self.uploaded_bone_count = n_bones;
            self.scene_dirty = false;
            self.intents_dirty = true;
        }

        if self.intents_dirty || has_walkways {
            self.d_intents.upload(stream, &scene.crowd.intents)?;
            // Mirror GPU one-shot impulse consumption on CPU so subsequent walkway intent uploads
            // never re-inject an already-applied poke impulse.
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

        self.d_grounds.upload(stream, &self.host_grounds)?;

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

        let block_x = 128u32;
        let grid_x = (n_actors as u32).div_ceil(block_x).max(1);
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

        // 6. Lightweight D2H of `states` (64 B/actor) for walkway steering, displacement-triggered
        //    IAS rebuilds, and Possess Body; optionally read back `world_joints` when possessing.
        self.d_states.download_into(stream, &mut scene.crowd.states)?;
        if sync_world_joints {
            self.d_world_joints
                .download_into(stream, &mut scene.crowd.world_joints)?;
        }
        stream
            .synchronize()
            .map_err(|e| format!("CUDA crowd step sync failed: {e}"))?;
        self.cpu_bones_stale = true;

        let mut max_disp_sq = 0.0f32;
        if self.last_rebuild_pos.len() == n_actors {
            for (st, prev) in scene.crowd.states.iter().zip(&self.last_rebuild_pos) {
                let dx = st.pos_yaw[0] - prev[0];
                let dy = st.pos_yaw[1] - prev[1];
                let dz = st.pos_yaw[2] - prev[2];
                let d2 = dx * dx + dy * dy + dz * dz;
                if d2.is_finite() && d2 > max_disp_sq {
                    max_disp_sq = d2;
                }
            }
        }
        let needs_full_ias_rebuild = max_disp_sq > CUDA_IAS_REBUILD_DISP_M * CUDA_IAS_REBUILD_DISP_M;
        if needs_full_ias_rebuild {
            for (st, prev) in scene.crowd.states.iter().zip(&mut self.last_rebuild_pos) {
                *prev = [st.pos_yaw[0], st.pos_yaw[1], st.pos_yaw[2]];
            }
        }

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
/// `SceneBuilt::step_crowd` verbatim.
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

/// Sample `GroundSamplePod`s for every actor in `scene.crowd` matching `CrowdSim::step_with_policy`.
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
        let mut backend = CudaCrowdBackend::new().expect("NVRTC compile actor_sim.cuh");

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
        let mut backend = CudaCrowdBackend::new().expect("NVRTC compile actor_sim.cuh");

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
}
