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
//! - [`NgxVulkan`] / [`FeatureHandle`] — Vulkan RAII wrappers (`Init_with_ProjectID` /
//!   `Shutdown1`, `CreateFeature` / `EvaluateFeature_C` / `ReleaseFeature`).
//!
//! **Conventions.** Every native handle owns its NGX object, holds an `Arc` of the parent
//! [`NgxVulkan`] session, and releases it on `Drop`. Non-Windows targets compile clean stubs
//! returning [`Error::Unsupported`].
//! **Used by** renderers, upscalers, and NGX GPU smoke tests.

pub mod error;
pub mod params;
pub mod types;
pub mod vulkan;

pub use error::{Error, NGX_RESULT_SUCCESS, Result, ResultCode};
pub use params::Parameters;
pub use types::{
    BufferFormat, DlssFeatureFlags, DlssPerfQuality, DlssQuality, Feature, Guid, ProjectId,
    RequiredExtensions,
};
pub use vulkan::{FeatureHandle, NgxVulkan, VulkanDeviceHandles};

#[cfg(target_os = "windows")]
pub use ngx_sys as sys;

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

        let unknown = ResultCode(-42);
        assert_eq!(unknown.name(), None);
        assert!(!unknown.is_success());
        assert!(unknown.to_string().contains("-42"));
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn sys_constants_match_wrapper_values() {
        assert_eq!(
            NGX_RESULT_SUCCESS,
            sys::NVSDK_NGX_Result_NVSDK_NGX_Result_Success
        );
        assert_eq!(
            DlssPerfQuality::MaxPerf.to_raw(),
            sys::NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_MaxPerf
        );
        assert_eq!(
            DlssPerfQuality::Dlaa.to_raw(),
            sys::NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_DLAA
        );
        assert_eq!(
            Feature::SuperSampling.to_raw(),
            sys::NVSDK_NGX_Feature_NVSDK_NGX_Feature_SuperSampling
        );
        assert_eq!(
            Feature::RayReconstruction.to_raw(),
            sys::NVSDK_NGX_Feature_NVSDK_NGX_Feature_RayReconstruction
        );
        let pid = ProjectId::from_guid_str("00000000-0000-0000-0000-000000000001").unwrap();
        assert_eq!(
            pid.engine_type(),
            sys::NVSDK_NGX_EngineType_NVSDK_NGX_ENGINE_TYPE_CUSTOM
        );
    }
}
