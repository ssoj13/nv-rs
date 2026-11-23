//! Raw NPP bindings: shared types plus one loader and shim set per NPP DLL.
//!
//! **What.** Generated at build time by `nv-bindgen` from the installed toolkit's `npp.h`. NPP
//! ships as eleven DLLs (`nppc` core, `npps` signals, nine `nppi*` image families); each is a
//! module here ([`nppc`], [`nppif`], ...) with its own `load()`, and every function is re-exported
//! at the crate root. A shim returns `NPP_ERROR` when its DLL is absent and
//! `NPP_NOT_IMPLEMENTED_ERROR` when a symbol is missing. `cudaStream_t` (in `NppStreamContext`) is
//! `cuda_bindings::CUstream_st`.
//! **Used by** a future safe `npp` crate.

nv_loader::split_exports_sys! {
    lib: "npp",
    parts: [
        { part: nppc, api: NppcApi, loader: nppc_api, dll: nv_loader::catalog::NPPC.spec },
        { part: npps, api: NppsApi, loader: npps_api, dll: nv_loader::catalog::NPPS.spec },
        { part: nppisu, api: NppisuApi, loader: nppisu_api, dll: nv_loader::catalog::NPPISU.spec },
        { part: nppidei, api: NppideiApi, loader: nppidei_api, dll: nv_loader::catalog::NPPIDEI.spec },
        { part: nppial, api: NppialApi, loader: nppial_api, dll: nv_loader::catalog::NPPIAL.spec },
        { part: nppicc, api: NppiccApi, loader: nppicc_api, dll: nv_loader::catalog::NPPICC.spec },
        { part: nppitc, api: NppitcApi, loader: nppitc_api, dll: nv_loader::catalog::NPPITC.spec },
        { part: nppim, api: NppimApi, loader: nppim_api, dll: nv_loader::catalog::NPPIM.spec },
        { part: nppif, api: NppifApi, loader: nppif_api, dll: nv_loader::catalog::NPPIF.spec },
        { part: nppist, api: NppistApi, loader: nppist_api, dll: nv_loader::catalog::NPPIST.spec },
        { part: nppig, api: NppigApi, loader: nppig_api, dll: nv_loader::catalog::NPPIG.spec },
    ],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_covers_every_family() {
        for f in [
            "nppGetLibVersion",
            "nppsAdd_32f_Ctx",
            "nppiFilter_8u_C1R_Ctx",
            "nppiResize_8u_C1R_Ctx",
            "nppiMean_8u_C1R_Ctx",
        ] {
            assert!(
                INVENTORY.contains(&format!("\"{f}\"")),
                "{f} missing from the inventory"
            );
        }
    }

    #[test]
    #[ignore = "GPU: needs the CUDA toolkit DLLs; run with --ignored"]
    fn every_dll_loads() {
        for (name, r) in [
            ("nppc", nppc::load()),
            ("npps", npps::load()),
            ("nppisu", nppisu::load()),
            ("nppidei", nppidei::load()),
            ("nppial", nppial::load()),
            ("nppicc", nppicc::load()),
            ("nppitc", nppitc::load()),
            ("nppim", nppim::load()),
            ("nppif", nppif::load()),
            ("nppist", nppist::load()),
            ("nppig", nppig::load()),
        ] {
            assert!(r.is_ok(), "{name}: {:?}", r.err());
        }
        // SAFETY: no arguments; returns a pointer to a static struct (or null if absent).
        assert!(!unsafe { nppGetLibVersion() }.is_null());
    }
}
