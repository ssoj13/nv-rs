//! Raw NGX (DLSS) bindings: SDK types and the NGX API, statically linked.
//!
//! **What.** Generated at build time by `nv-bindgen` from the NGX SDK headers (`nvsdk_ngx.h`,
//! `nvsdk_ngx_vk.h`; located by vcv-rs, never committed). NGX ships only static libraries, so —
//! unlike every other nv-rs sys crate — the entry points are plain `extern "C"` declarations linked
//! against `nvsdk_ngx_d.lib` at build time (PLAN D15). The library then loads the driver's NGX core
//! and the DLSS feature DLLs (`nvngx_dlss.dll`, shipped next to the application) at runtime.
//! Initialise with the `*_Init_with_ProjectID` functions and a project GUID of your own; no
//! NVIDIA-issued application id is needed. Windows x64 only for now.
//! **Licence.** The NGX/DLSS SDK is under the NVIDIA RTX SDKs licence: applications redistribute
//! the DLSS DLLs in object form and notify NVIDIA before release; see the SDK's licence file.
//! **Used by** a future safe `ngx` crate (DLSS-SR / DLSS-RR for render-rs).

// Generated code: lint levels differ per header.
#[allow(non_camel_case_types, non_upper_case_globals, non_snake_case, dead_code, missing_docs, unnecessary_transmutes, clippy::all, clippy::undocumented_unsafe_blocks)]
mod types {
    include!(concat!(env!("OUT_DIR"), "/ngx_types.rs"));
}
#[allow(non_camel_case_types, non_upper_case_globals, non_snake_case, missing_docs, clippy::all)]
mod externs {
    use super::types::*;
    include!(concat!(env!("OUT_DIR"), "/ngx_shims.rs"));
}
pub use externs::*;
pub use types::*;

/// Entry points and header sha256 of the generated bindings (coverage evidence).
pub const INVENTORY: &str = include_str!(concat!(env!("OUT_DIR"), "/ngx_inventory.json"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_init_entry_points() {
        for f in ["NVSDK_NGX_VULKAN_Init_with_ProjectID", "NVSDK_NGX_D3D12_Init_with_ProjectID", "NVSDK_NGX_CUDA_Init"] {
            assert!(INVENTORY.contains(&format!("\"{f}\"")), "{f} missing from the inventory");
        }
    }

    #[test]
    fn the_static_library_links() {
        // Taking the addresses forces the linker to resolve them from nvsdk_ngx_d.lib.
        let entries = [NVSDK_NGX_VULKAN_Init_with_ProjectID as *const (), NVSDK_NGX_D3D12_Init_with_ProjectID as *const ()];
        assert!(entries.iter().all(|p| !p.is_null()));
    }
}
