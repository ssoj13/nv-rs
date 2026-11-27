//! GPU camera composition runner executing `csg-rs`'s `PlanKind::Camera` v1 [`FramePlan`].
//!
//! What: wraps [`crate::frame_plan_runner::GpuFramePlan`] around
//! [`csg_scene::camera_gpu::camera_frame_plan`] (`camera_compose`), uploading per-frame
//! [`CameraModePod`] + [`PossessAttachPod`], zero-copy binding live device buffers from
//! `PlanKind::Crowd` (`BufId::CrowdBoneOuts`) and `PlanKind::Traffic`
//! (`BufId::TrafficInstanceOuts`), and reading back the 64-byte [`CameraRecordPod`] via
//! page-locked host memory with zero per-frame heap allocations.
//!
//! Why: possessed crowd bones and traffic vehicles advance on the GPU stream; composing the
//! camera directly on the same CUDA stream eliminates 1-frame pose lag while preserving exact
//! device-resident motion-blur history (`prev_eye`, `prev_q` with quaternion hemisphere
//! continuity) and loud fault signaling (`CAM_FAULT_MISSING_SOURCE` / `CAM_FAULT_UNKNOWN_MODE`).
//!
//! How: `GpuCameraComposer::new` validates and compiles `camera_frame_plan()` (version 1),
//! pre-allocates and zeroes `CameraModes`, `CameraAttaches`, and `CameraRecords` so
//! [`GpuCameraComposer::records_device_ptr`] is immediately valid for downstream GPU consumers
//! (raygen, lines, overlay), and allocates a [`PinnedHostBuf`] for the `ReadbackWhen::Always`
//! 64-byte record download. `compose_one` binds absent producers as `(nullptr, 0u)` per the
//! `BufClass::Bound` contract.
//!
//! Uses: `crate::frame_plan_runner::{GpuFramePlan, PinnedHostBuf}`, `csg_scene::camera_gpu`,
//! `csg_scene::frame_plan`, `cuda_core`.
//! Used by: `crate::renderer`.

use std::sync::Arc;

use csg_scene::camera_gpu::{
    CAMERA_FRAME_PLAN_VERSION, CameraFrameData, CameraModePod, CameraRecordPod, PossessAttachPod,
    camera_frame_plan,
};
use csg_scene::frame_plan::{BufId, PlanKind, ReadbackWhen};
use cuda_core::{Context as CudaContext, Stream};

use crate::frame_plan_runner::{GpuFramePlan, PinnedHostBuf};

/// Persistent GPU camera composer running `PlanKind::Camera` v1 (`camera_compose`) on CUDA.
///
/// `BufId::CameraRecords` stays device-resident across frames so `prev_eye` and `prev_q` update in
/// place with zero host upload and can be bound directly into OptiX raygen and overlay passes.
pub struct GpuCameraComposer {
    plan: GpuFramePlan,
    pinned_records: PinnedHostBuf<CameraRecordPod>,
}

impl GpuCameraComposer {
    /// Compiles and validates [`camera_frame_plan`] (version 1), pre-allocates zero-initialized
    /// device buffers for `camera_count` (`>= 1`) cameras, and allocates a page-locked readback
    /// buffer for the 64-byte `ReadbackWhen::Always` camera records.
    pub fn new(cuda: &Arc<CudaContext>, camera_count: usize) -> Result<Self, String> {
        if camera_count == 0 {
            return Err("GpuCameraComposer camera_count must be >= 1".to_string());
        }
        let mut plan = GpuFramePlan::new(
            cuda,
            camera_frame_plan(),
            CAMERA_FRAME_PLAN_VERSION,
            "camera_compose.cu",
        )?;

        // Pre-allocate and zero-initialize owned buffers (`CameraRecords` included) so
        // `records_device_ptr()` is valid immediately and history starts cleanly zeroed.
        let init_stream = cuda.default_stream();
        let init_sizes = CameraFrameData {
            cameras: camera_count,
            crowd_bones: 0,
            traffic_vehicles: 0,
        };
        plan.sync_sizes(&init_stream, &init_sizes)?;

        // Verify the plan declares `CameraRecords` as its every-frame readback.
        let has_records_readback = plan
            .plan()
            .readback_ids(ReadbackWhen::Always)
            .any(|id| id == BufId::CameraRecords);
        if !has_records_readback {
            return Err(
                "PlanKind::Camera missing BufId::CameraRecords in ReadbackWhen::Always".to_string(),
            );
        }

        let pinned_records = PinnedHostBuf::<CameraRecordPod>::new(cuda, camera_count)?;
        Ok(Self {
            plan,
            pinned_records,
        })
    }

    /// Composes camera slot `0` on `stream` from `mode` and `attach`, binding optional live
    /// device buffers `crowd_bones` (`(ptr, bone_count)`) and `traffic_outs`
    /// (`(ptr, vehicle_count)`) with zero host copies, and returns the completed 64-byte
    /// [`CameraRecordPod`] via page-locked readback.
    ///
    /// Note on `stream.synchronize()`: this method runs **only** while actively possessing a
    /// GPU-stepped crowd actor or traffic vehicle. The host must read the current-frame
    /// `CameraRecordPod` before launching OptiX raygen because `LaunchParams` (`752 B` C ABI)
    /// carries host-packed camera vectors after CPU `apply_possess_target` (damper + zoom offset)
    /// and `OvlCamera` upload; deferring readback by 1 frame would introduce 1-frame camera lag
    /// behind the GPU-stepped IAS. Eliminating this sync requires binding `BufId::CameraRecords`
    /// directly in `raytracer.cu` and moving possess damping onto the device.
    pub fn compose_one(
        &mut self,
        stream: &Stream,
        mode: CameraModePod,
        attach: PossessAttachPod,
        crowd_bones: Option<(u64, u32)>,
        traffic_outs: Option<(u64, u32)>,
    ) -> Result<CameraRecordPod, String> {
        self.plan.upload(stream, BufId::CameraModes, &[mode])?;
        self.plan.upload(stream, BufId::CameraAttaches, &[attach])?;

        // Enforce the `BufClass::Bound` absent-producer invariant: a null device pointer always
        // pairs with a `0` element count scalar (and vice versa).
        let (crowd_ptr, crowd_count) = match crowd_bones {
            Some((ptr, count)) if ptr != 0 && count > 0 => (ptr, count as usize),
            _ => (0u64, 0usize),
        };
        let (traffic_ptr, traffic_count) = match traffic_outs {
            Some((ptr, count)) if ptr != 0 && count > 0 => (ptr, count as usize),
            _ => (0u64, 0usize),
        };

        let frame_data = CameraFrameData {
            cameras: 1,
            crowd_bones: crowd_count,
            traffic_vehicles: traffic_count,
        };
        let bounds = [
            ((PlanKind::Crowd, BufId::CrowdBoneOuts), crowd_ptr),
            ((PlanKind::Traffic, BufId::TrafficInstanceOuts), traffic_ptr),
        ];

        self.plan
            .execute(stream, &frame_data, &frame_data, &[], &[], &bounds)?;

        for id in self.plan.plan().readback_ids(ReadbackWhen::Always) {
            match id {
                BufId::CameraRecords => {
                    self.plan.copy_to_pinned_buf_async(
                        stream,
                        id,
                        &mut self.pinned_records,
                        0,
                        1,
                    )?;
                }
                other => {
                    return Err(format!(
                        "PlanKind::Camera has unexpected ReadbackWhen::Always buffer {other:?}"
                    ));
                }
            }
        }
        stream
            .synchronize()
            .map_err(|e| format!("GpuCameraComposer stream synchronize failed: {e}"))?;
        self.pinned_records.read(0)
    }

    /// Returns the raw CUDA device pointer of persistent `BufId::CameraRecords`
    /// (`CameraRecordPod[camera_count]`) for direct GPU binding into raygen or overlay passes.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "device pointer reserved for Wave B GPU raygen and overlay passes"
        )
    )]
    #[inline]
    pub fn records_device_ptr(&self) -> u64 {
        self.plan.device_ptr(BufId::CameraRecords).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use csg_actors::BoneOutPod;
    use csg_scene::camera_gpu::{
        ATTACH_AUTO_LEVEL, ATTACH_CROWD_BONE, BONE_FLAG_VISIBLE, CAM_FAULT_MISSING_SOURCE,
        CAM_FAULT_NONE, CAM_FLAG_RESET_HISTORY, CAM_MODE_FREE,
        SceneCameraSources, compose_one as cpu_compose_one,
    };
    use cuda_bindings as cu;
    use cuda_core::DeviceBuffer;

    fn assert_record_close(label: &str, got: &CameraRecordPod, want: &CameraRecordPod) {
        assert_eq!(got.fault, want.fault, "{label}: fault mismatch");
        assert!(
            (got.vfov_deg - want.vfov_deg).abs() < 1e-5,
            "{label}: vfov mismatch got={} want={}",
            got.vfov_deg,
            want.vfov_deg
        );
        for i in 0..3 {
            assert!(
                (got.eye[i] - want.eye[i]).abs() < 1e-5,
                "{label}: eye[{i}] got={} want={}",
                got.eye[i],
                want.eye[i]
            );
            assert!(
                (got.prev_eye[i] - want.prev_eye[i]).abs() < 1e-5,
                "{label}: prev_eye[{i}] got={} want={}",
                got.prev_eye[i],
                want.prev_eye[i]
            );
        }
        for i in 0..4 {
            assert!(
                (got.q[i] - want.q[i]).abs() < 1e-5,
                "{label}: q[{i}] got={} want={}",
                got.q[i],
                want.q[i]
            );
            assert!(
                (got.prev_q[i] - want.prev_q[i]).abs() < 1e-5,
                "{label}: prev_q[{i}] got={} want={}",
                got.prev_q[i],
                want.prev_q[i]
            );
        }
    }

    #[test]
    #[ignore = "requires local NVIDIA GPU + CUDA driver + NVRTC"]
    fn gpu_camera_composer_matches_cpu_oracle_free_reset_and_crowd_possess() {
        let ctx = CudaContext::new(0).expect("CUDA context on device 0");
        ctx.bind_to_thread().expect("bind CUDA context");
        let stream = ctx.default_stream();
        let mut composer = GpuCameraComposer::new(&ctx, 1).expect("compile PlanKind::Camera v1");
        assert_ne!(
            composer.records_device_ptr(),
            0,
            "CameraRecords device pointer must be valid immediately after new()"
        );

        let empty_src = SceneCameraSources::default();
        let empty_attach = PossessAttachPod::default();

        // 1. Free mode with `CAM_FLAG_RESET_HISTORY`: prev_eye == eye and prev_q == q.
        let mode0 = CameraModePod {
            mode: CAM_MODE_FREE,
            flags: CAM_FLAG_RESET_HISTORY,
            vfov_deg: 58.0,
            pad: 0,
            eye: [12.5, 4.2, -9.0, 0.0],
            forward: [0.3, -0.15, 0.94, 0.0],
            up: [0.0, 1.0, 0.0, 0.0],
        };
        let cpu0 = cpu_compose_one(&mode0, &empty_attach, &empty_src, None);
        let gpu0 = composer
            .compose_one(&stream, mode0, empty_attach, None, None)
            .expect("GPU compose free+reset");
        assert_eq!(gpu0.fault, CAM_FAULT_NONE);
        assert_record_close("free+reset", &gpu0, &cpu0);

        // 2. Follow-up Free frame without reset: device history tracks prev_eye / prev_q in place.
        let mode1 = CameraModePod {
            mode: CAM_MODE_FREE,
            flags: 0,
            vfov_deg: 62.0,
            pad: 0,
            eye: [14.0, 4.5, -7.25, 0.0],
            forward: [-0.4, -0.05, 0.91, 0.0],
            up: [0.0, 1.0, 0.0, 0.0],
        };
        let cpu1 = cpu_compose_one(&mode1, &empty_attach, &empty_src, Some(&cpu0));
        let gpu1 = composer
            .compose_one(&stream, mode1, empty_attach, None, None)
            .expect("GPU compose free step");
        assert_eq!(gpu1.fault, CAM_FAULT_NONE);
        assert_record_close("free-step", &gpu1, &cpu1);

        // 3. Crowd-Bone Possess mode against a live device `BoneOutPod` buffer.
        let yaw = 0.65_f32;
        let (sy, cy) = yaw.sin_cos();
        let bones = [
            BoneOutPod {
                xform: [
                    cy, 0.0, sy, 24.0, // row 0
                    0.0, 1.0, 0.0, 1.65, // row 1
                    -sy, 0.0, cy, -11.5, // row 2
                ],
                compound_id: 3,
                user_id: 7,
                mat_override: 0,
                flags: BONE_FLAG_VISIBLE,
            },
            BoneOutPod {
                xform: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
                compound_id: 3,
                user_id: 8,
                mat_override: 0,
                flags: 0, // culled
            },
        ];
        let bone_bytes: &[u8] = bytemuck::cast_slice(&bones);
        let d_bones = DeviceBuffer::<u8>::zeroed(&stream, bone_bytes.len())
            .expect("allocate device bones");
        // SAFETY: `d_bones` owns `bone_bytes.len()` device bytes on `stream`.
        let rc = unsafe {
            cu::cuMemcpyHtoDAsync_v2(
                d_bones.cu_deviceptr(),
                bone_bytes.as_ptr().cast(),
                bone_bytes.len(),
                stream.cu_stream(),
            )
        };
        assert_eq!(rc, cu::cudaError_enum_CUDA_SUCCESS, "H2D bone upload failed");

        let crowd_src = SceneCameraSources {
            crowd_bones: Some(&bones),
            traffic_outs: None,
        };
        let attach_vis = PossessAttachPod {
            source: ATTACH_CROWD_BONE,
            index: 0,
            flags: ATTACH_AUTO_LEVEL,
            pad: 0,
            mount_eye: [0.0, 0.18, 0.12, 0.0],
            mount_look: [0.0, 0.0, 1.0, 0.0],
            mount_up: [0.0, 1.0, 0.0, 0.0],
            gimbal_deg: [12.0, -5.0, 2.0, 0.0],
            free_yaw_pitch: [0.22, -0.08, 0.0, 0.0],
        };
        let mode_possess = CameraModePod::possess(65.0, 0);
        let cpu2 = cpu_compose_one(&mode_possess, &attach_vis, &crowd_src, Some(&cpu1));
        let gpu2 = composer
            .compose_one(
                &stream,
                mode_possess,
                attach_vis,
                Some((d_bones.cu_deviceptr(), bones.len() as u32)),
                None,
            )
            .expect("GPU compose crowd possess");
        assert_eq!(gpu2.fault, CAM_FAULT_NONE);
        assert_record_close("crowd-possess", &gpu2, &cpu2);

        // 4. Culled bone triggers `CAM_FAULT_MISSING_SOURCE` and holds previous pose verbatim.
        let attach_culled = PossessAttachPod {
            index: 1,
            ..attach_vis
        };
        let cpu3 = cpu_compose_one(&mode_possess, &attach_culled, &crowd_src, Some(&cpu2));
        let gpu3 = composer
            .compose_one(
                &stream,
                mode_possess,
                attach_culled,
                Some((d_bones.cu_deviceptr(), bones.len() as u32)),
                None,
            )
            .expect("GPU compose culled crowd possess");
        assert_eq!(gpu3.fault, CAM_FAULT_MISSING_SOURCE);
        assert_record_close("crowd-possess-culled", &gpu3, &cpu3);
    }
}
