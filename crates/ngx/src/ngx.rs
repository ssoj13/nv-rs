//! Safe NVIDIA NGX (DLSS) wrapper over `ngx-sys`.
//!
//! **What.**
//! - [`ProjectId`] / [`Guid`] — project identifier (`NVSDK_NGX_ENGINE_TYPE_CUSTOM`, PLAN D13) with
//!   optional engine name and version.
//! - [`Error`] / [`ResultCode`] — `thiserror` error enum and `NVSDK_NGX_Result` status wrapper.
//! - [`DlssPerfQuality`] ([`DlssQuality`]), [`Feature`], [`BufferFormat`], [`DlssFeatureFlags`] —
//!   strongly typed DLSS/NGX enums and flags.
//! - [`Parameters`] — RAII `NVSDK_NGX_Parameter` wrapper (`AllocateParameters` /
//!   `GetCapabilityParameters` / `DestroyParameters`) with typed setters and getters.
//! - [`NgxVulkan`] / [`FeatureHandle`] — Vulkan RAII wrappers (`Init` / `Init_with_ProjectID` /
//!   `Shutdown1`, `CreateFeature` / `EvaluateFeature_C` / `ReleaseFeature`).
//!
//! **Conventions.** Every native handle owns its NGX object, holds an `Arc` of the parent
//! [`NgxVulkan`] session, and releases it on `Drop`; teardown failures are kept with their raw
//! status in `cuda_core_ext::teardown` and drained by [`take_teardown_errors`]. NGX is
//! documented as not thread-safe (`nvsdk_ngx.h:18-19`, `nvsdk_ngx_vk.h:16-17`), so every NGX
//! entry point runs under one process-wide lock. Targets without the NGX SDK compile clean stubs
//! returning [`Error::Unsupported`].
//! **Used by** renderers, upscalers, and NGX GPU smoke tests.

pub mod error;
pub mod params;
pub mod types;
pub mod vulkan;

pub use error::{Error, NGX_RESULT_SUCCESS, Result, ResultCode};
pub use params::Parameters;
pub use types::{
    BufferFormat, DlssDenoiseMode, DlssDepthType, DlssFeatureFlags, DlssPerfQuality, DlssQuality,
    DlssRoughnessMode, DlssRrPreset, Feature, Guid, ProjectId, RequiredExtensions,
};
pub use vulkan::{
    DlssOptimalSettings, DlssRrAvailability, DlssRrCreateParams, DlssRrEvalParams, FeatureHandle,
    NgxVulkan, VkImageResourceInfo, VulkanDeviceHandles,
};

#[cfg(has_ngx_sdk)]
pub use ngx_sys as sys;

/// Converts an FFI `NVSDK_NGX_Result` (`i32` on MSVC, `u32` on Linux GCC/Clang) to `i32`
/// without triggering `clippy::unnecessary_cast` on either target.
#[cfg(has_ngx_sdk)]
#[inline(always)]
pub(crate) fn check_sys(call: &'static str, rc: sys::NVSDK_NGX_Result) -> Result<()> {
    error::check(call, i32::from_ne_bytes(rc.to_ne_bytes()))
}

/// Process-wide NGX state, owned by the lock that serialises every NGX entry point.
///
/// The lock is global rather than per session because NGX state is process-global and the
/// header contract ("Methods in this library are NOT thread safe. It is up to the client to
/// ensure that thread safety is enforced as needed", `nvsdk_ngx_vk.h:16-17`) also covers the
/// session-less [`NgxVulkan::required_extensions`] and sessions on different `VkDevice`s.
#[cfg(has_ngx_sdk)]
pub(crate) struct NgxGlobal {
    /// `VkDevice` addresses that currently own a live [`NgxVulkan`] session.
    live_devices: Vec<usize>,
}

#[cfg(has_ngx_sdk)]
static NGX_GLOBAL: std::sync::Mutex<NgxGlobal> = std::sync::Mutex::new(NgxGlobal {
    live_devices: Vec::new(),
});

#[cfg(has_ngx_sdk)]
impl NgxGlobal {
    /// Reserves `device` for a new session.
    ///
    /// `NVSDK_NGX_VULKAN_Shutdown1(device)` shuts down the instance for that device
    /// (`nvsdk_ngx_vk.h:270-272`), so a second live session on the same device would have NGX
    /// shut down underneath its features when the first session drops.
    pub(crate) fn claim_device(&mut self, device: usize) -> Result<()> {
        if self.live_devices.contains(&device) {
            return Err(Error::InvalidArgument(
                "NGX is already initialized on this VkDevice; clone the existing NgxVulkan",
            ));
        }
        self.live_devices.push(device);
        Ok(())
    }

    /// Releases a device reserved by [`NgxGlobal::claim_device`].
    pub(crate) fn release_device(&mut self, device: usize) {
        self.live_devices.retain(|&d| d != device);
    }

    /// Keeps a failed teardown status from a `Drop` impl in `cuda_core_ext::teardown`
    /// (`Library::Ngx`, the raw `NVSDK_NGX_Result`) for [`take_teardown_errors`]. Records only
    /// what [`check_sys`] reports as a failure.
    pub(crate) fn record_teardown(call: &'static str, rc: sys::NVSDK_NGX_Result) {
        if check_sys(call, rc).is_err() {
            cuda_core_ext::teardown::record(
                cuda_core_ext::teardown::Library::Ngx,
                call,
                i64::from(i32::from_ne_bytes(rc.to_ne_bytes())),
            );
        }
    }
}

/// Runs `f` while holding the process-wide NGX lock, with access to the shared NGX state.
///
/// Callers must not re-enter (`f` must not call another `ngx_*` helper or drop an NGX handle):
/// the lock is not reentrant.
#[cfg(has_ngx_sdk)]
pub(crate) fn ngx_locked<R>(f: impl FnOnce(&mut NgxGlobal) -> R) -> R {
    // Every update of `NgxGlobal` is a single push/retain/assignment, so a panic in another
    // holder cannot leave it inconsistent; recover the guard instead of propagating the poison.
    let mut guard = NGX_GLOBAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    f(&mut guard)
}

/// Runs one NGX entry point (`f`) while holding the process-wide NGX lock.
#[cfg(has_ngx_sdk)]
pub(crate) fn ngx_call<R>(f: impl FnOnce() -> R) -> R {
    ngx_locked(|_| f())
}

/// Drains the failures reported by NGX teardown calls that ran in `Drop`
/// (`NVSDK_NGX_VULKAN_DestroyParameters`, `NVSDK_NGX_VULKAN_ReleaseFeature`,
/// `NVSDK_NGX_VULKAN_Shutdown1`), oldest first, each with its raw `NVSDK_NGX_Result`.
///
/// `Drop` cannot return an error, so the statuses are kept in the process-wide
/// `cuda_core_ext::teardown` sink; this is `cuda_core_ext::teardown::take(Some(Library::Ngx))`.
/// Always empty on targets without the NGX SDK.
pub fn take_teardown_errors() -> cuda_core_ext::teardown::Drained {
    cuda_core_ext::teardown::take(Some(cuda_core_ext::teardown::Library::Ngx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::check;

    #[test]
    fn guid_parse_and_roundtrip() {
        let raw = "A1B2C3D4-E5F6-4789-ABCD-0123456789EF";
        let guid = Guid::parse(raw).expect("valid GUID");
        assert_eq!(guid.to_string(), "a1b2c3d4-e5f6-4789-abcd-0123456789ef");
        assert_eq!(Guid::from_bytes(*guid.as_bytes()), guid);
        assert_eq!(
            guid.as_bytes(),
            &[
                0xa1, 0xb2, 0xc3, 0xd4, 0xe5, 0xf6, 0x47, 0x89, 0xab, 0xcd, 0x01, 0x23, 0x45, 0x67,
                0x89, 0xef
            ]
        );
    }

    #[test]
    fn guid_rejects_malformed_inputs() {
        for bad in [
            "",
            "a1b2c3d4e5f64789abcd0123456789ef",
            "a1b2c3d4-e5f6-4789-abcd-0123456789eg",
            "a1b2c3d4_e5f6-4789-abcd-0123456789ef",
            "a1b2c3d4-e5f6-4789-abcd-0123456789ef0",
        ] {
            assert!(
                matches!(Guid::parse(bad), Err(Error::InvalidArgument(_))),
                "accepted {bad}"
            );
        }
    }

    #[test]
    fn project_id_custom_engine_and_metadata() {
        let pid = ProjectId::from_guid_str("12345678-1234-4234-8234-1234567890ab")
            .expect("valid GUID")
            .with_engine_name("nv-rs-engine")
            .expect("valid engine_name")
            .with_engine_version("0.1.0")
            .expect("valid engine_version");

        assert_eq!(pid.engine_type(), 0);
        assert_eq!(pid.engine_name(), Some("nv-rs-engine"));
        assert_eq!(pid.engine_version(), Some("0.1.0"));
        assert_eq!(
            pid.guid_cstr().to_str().unwrap(),
            "12345678-1234-4234-8234-1234567890ab"
        );

        assert!(matches!(
            ProjectId::from_guid_str("12345678-1234-4234-8234-1234567890ab")
                .unwrap()
                .with_engine_version("1.0\0bad"),
            Err(Error::InvalidArgument(_))
        ));
    }

    #[test]
    fn dlss_quality_mapping_roundtrip() {
        let cases = [
            (DlssPerfQuality::MaxPerf, 0),
            (DlssPerfQuality::Balanced, 1),
            (DlssPerfQuality::MaxQuality, 2),
            (DlssPerfQuality::UltraPerformance, 3),
            (DlssPerfQuality::UltraQuality, 4),
            (DlssPerfQuality::Dlaa, 5),
        ];
        for (variant, raw) in cases {
            let alias: DlssQuality = variant;
            assert_eq!(alias.to_raw(), raw);
            assert_eq!(DlssPerfQuality::from_raw(raw), Some(variant));
        }
        assert_eq!(DlssPerfQuality::from_raw(99), None);
    }

    #[test]
    fn feature_and_buffer_format_roundtrip() {
        for feature in [
            Feature::SuperSampling,
            Feature::InPainting,
            Feature::ImageSuperResolution,
            Feature::SlowMotion,
            Feature::VideoSuperResolution,
            Feature::ImageSignalProcessing,
            Feature::DeepResolve,
            Feature::FrameGeneration,
            Feature::DeepDvc,
            Feature::RayReconstruction,
        ] {
            assert_eq!(Feature::from_raw(feature.to_raw()), Some(feature));
        }
        assert_eq!(Feature::from_raw(-1), None);

        for fmt in [
            BufferFormat::Unknown,
            BufferFormat::Rgb8Ui,
            BufferFormat::Rgb16F,
            BufferFormat::Rgb32F,
            BufferFormat::Rgba8Ui,
            BufferFormat::Rgba16F,
            BufferFormat::Rgba32F,
        ] {
            assert_eq!(BufferFormat::from_raw(fmt.to_raw()), Some(fmt));
        }
        assert_eq!(BufferFormat::from_raw(42), None);
    }

    #[test]
    fn dlss_feature_flags_combine() {
        let mut flags = DlssFeatureFlags::IS_HDR | DlssFeatureFlags::MV_LOW_RES;
        flags |= DlssFeatureFlags::DEPTH_INVERTED;
        assert!(flags.contains(DlssFeatureFlags::IS_HDR));
        assert!(flags.contains(DlssFeatureFlags::MV_LOW_RES));
        assert!(flags.contains(DlssFeatureFlags::DEPTH_INVERTED));
        assert!(!flags.contains(DlssFeatureFlags::AUTO_EXPOSURE));
    }

    #[test]
    fn error_and_result_code_mapping() {
        assert!(check("ok", NGX_RESULT_SUCCESS).is_ok());

        let err = check("NVSDK_NGX_VULKAN_Init", -1_160_773_631).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("NVSDK_NGX_VULKAN_Init"), "unexpected: {msg}");
        assert!(msg.contains("FeatureNotSupported"), "unexpected: {msg}");
        assert!(msg.contains("0xBAD00001"), "unexpected: {msg}");

        // 0xBAD000FF: in the failure class (`NVSDK_NGX_FAILED`) but not a named code.
        let unknown = ResultCode(-1_160_773_377);
        assert_eq!(unknown.name(), None);
        assert!(!unknown.is_success());
        assert!(unknown.to_string().contains("-1160773377"));
    }

    #[cfg(has_ngx_sdk)]
    #[test]
    fn sys_constants_match_wrapper_values() {
        assert_eq!(
            NGX_RESULT_SUCCESS,
            i32::from_ne_bytes(sys::NVSDK_NGX_Result_NVSDK_NGX_Result_Success.to_ne_bytes())
        );
        assert_eq!(
            DlssPerfQuality::MaxPerf.to_raw(),
            i32::from_ne_bytes(
                sys::NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_MaxPerf.to_ne_bytes()
            )
        );
        assert_eq!(
            DlssPerfQuality::Dlaa.to_raw(),
            i32::from_ne_bytes(
                sys::NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_DLAA.to_ne_bytes()
            )
        );
        assert_eq!(
            Feature::SuperSampling.to_raw(),
            i32::from_ne_bytes(
                sys::NVSDK_NGX_Feature_NVSDK_NGX_Feature_SuperSampling.to_ne_bytes()
            )
        );
        assert_eq!(
            Feature::RayReconstruction.to_raw(),
            i32::from_ne_bytes(
                sys::NVSDK_NGX_Feature_NVSDK_NGX_Feature_RayReconstruction.to_ne_bytes()
            )
        );
        let pid = ProjectId::from_guid_str("00000000-0000-0000-0000-000000000001").unwrap();
        assert_eq!(
            pid.engine_type(),
            i32::from_ne_bytes(
                sys::NVSDK_NGX_EngineType_NVSDK_NGX_ENGINE_TYPE_CUSTOM.to_ne_bytes()
            )
        );
    }

    #[test]
    fn vulkan_init_rejects_null_handles_or_reports_unsupported() {
        let null_handles = VulkanDeviceHandles {
            instance: std::ptr::null_mut(),
            physical_device: std::ptr::null_mut(),
            device: std::ptr::null_mut(),
            get_instance_proc_addr: std::ptr::null(),
            get_device_proc_addr: std::ptr::null(),
        };
        let pid = ProjectId::from_guid_str("00000000-0000-0000-0000-000000000001").unwrap();
        let data_path = std::path::Path::new(".");
        // SAFETY: All pointers are null; both functions reject null handles before dereferencing
        // (or return `Error::Unsupported` on non-Windows targets).
        let res_pid = unsafe { NgxVulkan::init(&pid, data_path, null_handles) };
        // SAFETY: Same as above.
        let res_app = unsafe { NgxVulkan::init_with_app_id(12345, data_path, null_handles) };
        #[cfg(has_ngx_sdk)]
        {
            assert!(matches!(res_pid, Err(Error::InvalidArgument(_))));
            assert!(matches!(res_app, Err(Error::InvalidArgument(_))));
        }
        #[cfg(not(has_ngx_sdk))]
        {
            assert!(matches!(res_pid, Err(Error::Unsupported(_))));
            assert!(matches!(res_app, Err(Error::Unsupported(_))));
        }
    }

    #[test]
    fn dlss_helper_structs_construct_and_default() {
        let info = VkImageResourceInfo {
            image_view: std::ptr::null_mut(),
            image: std::ptr::null_mut(),
            format: 97, // VK_FORMAT_R16G16B16A16_SFLOAT
            width: 1920,
            height: 1080,
            read_write: true,
        };
        assert_eq!(info.width, 1920);
        assert_eq!(info.height, 1080);
        assert!(info.read_write);

        let settings = DlssOptimalSettings::default();
        assert_eq!(settings.render_width, 0);
        assert_eq!(settings.sharpness, 0.0);

        let rr_create = DlssRrCreateParams {
            render_width: 960,
            render_height: 540,
            out_width: 1920,
            out_height: 1080,
            quality: DlssPerfQuality::MaxQuality,
            flags: DlssFeatureFlags::IS_HDR | DlssFeatureFlags::MV_LOW_RES,
            roughness_mode: DlssRoughnessMode::Packed,
            depth_type: DlssDepthType::Linear,
            preset: DlssRrPreset::Default,
        };
        assert_eq!(rr_create.roughness_mode, DlssRoughnessMode::Packed);
        assert_eq!(rr_create.depth_type, DlssDepthType::Linear);

        let rr_eval = DlssRrEvalParams {
            color: info,
            depth: info,
            motion: info,
            diffuse_albedo: info,
            specular_albedo: info,
            normals: info,
            roughness: None,
            output: info,
            render_width: 960,
            render_height: 540,
            jitter_x: 0.25,
            jitter_y: -0.25,
            mv_scale_x: 1.0,
            mv_scale_y: 1.0,
            pre_exposure: 1.0,
            exposure_scale: 1.0,
            reset: true,
            world_to_view: None,
            view_to_clip: None,
        };
        assert!(rr_eval.reset);
        assert!(rr_eval.roughness.is_none());
    }

    #[test]
    fn dlss_rr_enum_roundtrips() {
        for (variant, raw) in [(DlssDenoiseMode::Off, 0), (DlssDenoiseMode::DlUnified, 1)] {
            assert_eq!(variant.to_raw(), raw);
            assert_eq!(DlssDenoiseMode::from_raw(raw), Some(variant));
        }
        assert_eq!(DlssDenoiseMode::default(), DlssDenoiseMode::DlUnified);
        assert_eq!(DlssDenoiseMode::from_raw(2), None);

        for (variant, raw) in [
            (DlssRoughnessMode::Unpacked, 0),
            (DlssRoughnessMode::Packed, 1),
        ] {
            assert_eq!(variant.to_raw(), raw);
            assert_eq!(DlssRoughnessMode::from_raw(raw), Some(variant));
        }
        assert_eq!(DlssRoughnessMode::default(), DlssRoughnessMode::Unpacked);
        assert_eq!(DlssRoughnessMode::from_raw(2), None);

        for (variant, raw) in [(DlssDepthType::Linear, 0), (DlssDepthType::Hw, 1)] {
            assert_eq!(variant.to_raw(), raw);
            assert_eq!(DlssDepthType::from_raw(raw), Some(variant));
        }
        assert_eq!(DlssDepthType::default(), DlssDepthType::Linear);
        assert_eq!(DlssDepthType::from_raw(2), None);

        for (variant, raw) in [
            (DlssRrPreset::Default, 0),
            (DlssRrPreset::D, 4),
            (DlssRrPreset::E, 5),
            (DlssRrPreset::F, 6),
        ] {
            assert_eq!(variant.to_raw(), raw);
            assert_eq!(DlssRrPreset::from_raw(raw), Some(variant));
        }
        assert_eq!(DlssRrPreset::default(), DlssRrPreset::Default);
        assert_eq!(DlssRrPreset::from_raw(1), None);
        assert_eq!(DlssRrPreset::from_raw(99), None);
    }

    #[test]
    fn dlss_rr_keys_match_sdk_defs() {
        assert_eq!(
            params::keys::RAY_RECONSTRUCTION_AVAILABLE,
            "SuperSamplingDenoising.Available"
        );
        assert_eq!(
            params::keys::DLSSD_OPTIMAL_SETTINGS_CALLBACK,
            "DLSSDOptimalSettingsCallback"
        );
        assert_eq!(params::keys::DLSS_DENOISE_MODE, "DLSS.Denoise.Mode");
        assert_eq!(params::keys::DLSS_ROUGHNESS_MODE, "DLSS.Roughness.Mode");
        assert_eq!(params::keys::DLSS_USE_HW_DEPTH, "DLSS.Use.HW.Depth");
        assert_eq!(
            params::keys::DLSS_INPUT_DIFFUSE_ALBEDO,
            "DLSS.Input.DiffuseAlbedo"
        );
        assert_eq!(
            params::keys::DLSS_INPUT_SPECULAR_ALBEDO,
            "DLSS.Input.SpecularAlbedo"
        );
        assert_eq!(params::keys::GBUFFER_NORMALS, "GBuffer.Normals");
        assert_eq!(params::keys::GBUFFER_ROUGHNESS, "GBuffer.Roughness");
        assert_eq!(params::keys::WORLD_TO_VIEW_MATRIX, "WorldToViewMatrix");
        assert_eq!(params::keys::VIEW_TO_CLIP_MATRIX, "ViewToClipMatrix");
    }

    /// `nvsdk_ngx_defs_dlssd.h:152-155`.
    #[test]
    fn dlss_rr_availability_keys_match_sdk_defs() {
        assert_eq!(
            params::keys::RAY_RECONSTRUCTION_NEEDS_UPDATED_DRIVER,
            "SuperSamplingDenoising.NeedsUpdatedDriver"
        );
        assert_eq!(
            params::keys::RAY_RECONSTRUCTION_MIN_DRIVER_VERSION_MAJOR,
            "SuperSamplingDenoising.MinDriverVersionMajor"
        );
        assert_eq!(
            params::keys::RAY_RECONSTRUCTION_MIN_DRIVER_VERSION_MINOR,
            "SuperSamplingDenoising.MinDriverVersionMinor"
        );
        assert_eq!(
            params::keys::RAY_RECONSTRUCTION_FEATURE_INIT_RESULT,
            "SuperSamplingDenoising.FeatureInitResult"
        );
    }

    /// The session handle stays shareable across threads (nv-demo, nv-py) because every NGX entry
    /// point is serialised by the process-wide lock rather than by thread confinement.
    #[test]
    fn ngx_vulkan_remains_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<NgxVulkan>();
    }

    #[cfg(has_ngx_sdk)]
    #[test]
    fn ngx_call_serialises_concurrent_callers() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let inside = AtomicUsize::new(0);
        let peak = AtomicUsize::new(0);
        std::thread::scope(|s| {
            for _ in 0..8 {
                s.spawn(|| {
                    for _ in 0..50 {
                        crate::ngx_call(|| {
                            let now = inside.fetch_add(1, Ordering::SeqCst) + 1;
                            peak.fetch_max(now, Ordering::SeqCst);
                            std::thread::yield_now();
                            inside.fetch_sub(1, Ordering::SeqCst);
                        });
                    }
                });
            }
        });
        assert_eq!(peak.load(Ordering::SeqCst), 1);
    }

    #[cfg(has_ngx_sdk)]
    #[test]
    fn second_session_on_same_device_is_rejected() {
        // Fake addresses never handed to NGX; unique to this test.
        let (dev_a, dev_b) = (0xDEAD_0000_usize, 0xDEAD_0040_usize);
        crate::ngx_locked(|g| {
            assert!(g.claim_device(dev_a).is_ok());
            assert!(matches!(
                g.claim_device(dev_a),
                Err(Error::InvalidArgument(_))
            ));
            assert!(g.claim_device(dev_b).is_ok());
            g.release_device(dev_a);
            assert!(g.claim_device(dev_a).is_ok());
            g.release_device(dev_a);
            g.release_device(dev_b);
        });
    }

    #[cfg(has_ngx_sdk)]
    #[test]
    fn teardown_failure_is_recorded_not_dropped() {
        // Only this test records `Library::Ngx` in this test binary, so the drains are exact.
        take_teardown_errors();
        NgxGlobal::record_teardown(
            "NVSDK_NGX_VULKAN_ReleaseFeature",
            sys::NVSDK_NGX_Result_NVSDK_NGX_Result_Success,
        );
        assert!(take_teardown_errors().is_empty());
        NgxGlobal::record_teardown(
            "NVSDK_NGX_VULKAN_ReleaseFeature",
            sys::NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_OutOfDate,
        );
        let drained = take_teardown_errors();
        assert_eq!(drained.lost, 0);
        assert_eq!(
            drained.errors,
            vec![cuda_core_ext::teardown::TeardownError {
                library: cuda_core_ext::teardown::Library::Ngx,
                op: "NVSDK_NGX_VULKAN_ReleaseFeature",
                status: i64::from(ResultCode::OUT_OF_DATE.0),
            }]
        );
        assert!(take_teardown_errors().is_empty(), "drained exactly once");
    }

    #[cfg(has_ngx_sdk)]
    #[test]
    fn wchar_nul_encodes_ascii_path_with_terminator() {
        let wide = crate::vulkan::to_wchar_nul(std::path::Path::new("ngx/logs"));
        assert_eq!(wide.last().copied(), Some(0));
        assert_eq!(wide.len(), "ngx/logs".len() + 1);
    }
}
