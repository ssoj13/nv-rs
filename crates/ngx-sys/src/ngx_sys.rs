//! Raw NGX (DLSS) bindings: SDK types and the NGX API, statically linked.
//!
//! **What.** Generated at build time by `nv-bindgen` from the NGX SDK headers (`nvsdk_ngx.h`,
//! `nvsdk_ngx_vk.h`; located by vcv-rs, never committed). NGX ships only static libraries, so —
//! unlike every other nv-rs sys crate — the entry points are plain `extern "C"` declarations linked
//! against `nvsdk_ngx_d.lib` (Windows) or `libnvsdk_ngx.a` (Linux x86_64, when `NGX_SDK_ROOT` /
//! `NV_SDK_DIRS` provides it, gated by `cfg(has_ngx_sdk)`) at build time (PLAN D15). The library
//! then loads the driver's NGX core and the DLSS feature DLLs/shared objects at runtime.
//! Initialise with `*_Init_with_ProjectID` (and a project GUID of your own) or `*_Init` (with an
//! NVIDIA-issued `ApplicationId`).
//! **Licence.** The NGX/DLSS SDK is under the NVIDIA RTX SDKs licence: applications redistribute
//! the DLSS DLLs in object form and notify NVIDIA before release; see the SDK's licence file.
//! **Used by** the safe `ngx` crate (DLSS-SR / DLSS-RR for render-rs).
#![cfg(any(target_os = "windows", target_os = "linux"))]

// Generated code: lint levels differ per header.
#[cfg(has_ngx_sdk)]
#[allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    dead_code,
    missing_docs,
    unnecessary_transmutes,
    clippy::all,
    clippy::undocumented_unsafe_blocks
)]
mod types {
    include!(concat!(env!("OUT_DIR"), "/ngx_types.rs"));
}
#[cfg(has_ngx_sdk)]
#[allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    missing_docs,
    clippy::all
)]
mod externs {
    use super::types::*;
    include!(concat!(env!("OUT_DIR"), "/ngx_shims.rs"));
}
#[cfg(has_ngx_sdk)]
pub use externs::*;
#[cfg(has_ngx_sdk)]
pub use types::*;

/// Entry points and header sha256 of the generated bindings (coverage evidence).
#[cfg(has_ngx_sdk)]
pub const INVENTORY: &str = include_str!(concat!(env!("OUT_DIR"), "/ngx_inventory.json"));

/// Build-time NGX SDK runtime DLL/DSO directory discovered by `vcv-rs` (`None` when empty).
#[cfg(has_ngx_sdk)]
pub const SDK_BIN: Option<&str> = match option_env!("NV_RS_SDK_BIN") {
    Some(s) if s.is_empty() => None,
    some => some,
};

#[cfg(all(test, has_ngx_sdk))]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_init_entry_points() {
        for f in [
            "NVSDK_NGX_VULKAN_Init_with_ProjectID",
            "NVSDK_NGX_VULKAN_Init",
            "NVSDK_NGX_CUDA_Init",
        ] {
            assert!(
                INVENTORY.contains(&format!("\"{f}\"")),
                "{f} missing from the inventory"
            );
        }
        #[cfg(target_os = "windows")]
        assert!(
            INVENTORY.contains("\"NVSDK_NGX_D3D12_Init_with_ProjectID\""),
            "NVSDK_NGX_D3D12_Init_with_ProjectID missing from the inventory"
        );
    }

    #[test]
    fn the_static_library_links() {
        // Taking the addresses forces the linker to resolve them from the NGX static library.
        #[cfg(target_os = "windows")]
        let entries = [
            NVSDK_NGX_VULKAN_Init_with_ProjectID as *const (),
            NVSDK_NGX_VULKAN_Init as *const (),
            NVSDK_NGX_D3D12_Init_with_ProjectID as *const (),
        ];
        #[cfg(target_os = "linux")]
        let entries = [
            NVSDK_NGX_VULKAN_Init_with_ProjectID as *const (),
            NVSDK_NGX_VULKAN_Init as *const (),
        ];
        assert!(entries.iter().all(|p| !p.is_null()));
    }
}
