//! T2 GPU smoke test for `ngx` (`NgxVulkan::required_extensions`).
//!
//! **What.** Exercises the live `nvsdk_ngx_d.lib` -> `nvngx.dll` extension query when an NVIDIA
//! driver / NGX runtime is present on Windows.
//! **Why.** Verifies end-to-end static linking and runtime NGX core discovery without requiring a
//! full swapchain/render-pass harness.
//! **How.** Marked `#[ignore]` per PLAN.md T2 policy; gracefully skips if `NVSDK_NGX_Result_FAIL_NotImplemented`
//! or `PlatformError` is returned when no NGX runtime DLL is installed.

#[test]
#[ignore = "T2 GPU test: requires NVIDIA driver and NGX runtime"]
fn ngx_vulkan_required_extensions_smoke() {
    match ngx::NgxVulkan::required_extensions() {
        Ok(exts) => {
            assert!(
                exts.instance_extensions
                    .iter()
                    .any(|e| e.starts_with("VK_")),
                "expected Vulkan instance extensions, got {:?}",
                exts.instance_extensions
            );
        }
        Err(ngx::Error::Unsupported(reason)) => {
            eprintln!("skipping on unsupported platform: {reason}");
        }
        Err(e) => panic!("NgxVulkan::required_extensions failed unexpectedly: {e}"),
    }
}
