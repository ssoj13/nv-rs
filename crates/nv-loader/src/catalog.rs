//! The single catalog of every NVIDIA runtime library nv-rs loads, probes or stages.
//!
//! **What.** One [`CatalogEntry`] constant per library (`CUBLAS`, `CUDNN`, `NPPIF`, ...) and
//! [`ALL`], the list of every one. Each entry is the [`DllSpec`] the loader searches with.
//!
//! **Why.** The specs used to be copied into each sys crate, `diag::probe_all` and
//! `xtask stage-dlls`, and the copies had already drifted (different Linux sonames, extra Windows
//! names, renamed libraries, missing entries). Now there is one owner.
//!
//! **How.** Plain `const` data. SDK entries carry no build hint: a sys crate adds its own
//! build-time SDK directory with [`DllSpec::with_hint`] (debug builds only), which nv-loader cannot
//! know. Linux sonames list the major version the bindings are generated for first.
//!
//! **Used by** every `*-sys` crate (`pub const DLL`), [`crate::diag::probe_all`] and
//! `xtask stage-dlls`.

use crate::{DllSpec, Group};

/// One runtime library: its catalog name (equal to `spec.lib`) and how it is found.
#[derive(Clone, Copy, Debug)]
pub struct CatalogEntry {
    /// Short library name, the same as `spec.lib` (`"cufft"`).
    pub name: &'static str,
    /// Where it ships and what its files are called.
    pub spec: DllSpec,
}

const fn entry(
    name: &'static str,
    group: Group,
    windows: &'static [&'static str],
    linux: &'static [&'static str],
) -> CatalogEntry {
    CatalogEntry {
        name,
        spec: DllSpec {
            lib: name,
            group,
            windows,
            linux,
        },
    }
}

const fn sdk(root_var: &'static str) -> Group {
    Group::Sdk {
        root_var,
        hint: None,
    }
}

// Display driver.

/// The CUDA driver API (loaded by `cuda-bindings`; probed by diagnostics).
pub const NVCUDA: CatalogEntry = entry(
    "nvcuda",
    Group::Driver,
    &["nvcuda.dll"],
    &["libcuda.so.1", "libcuda.so"],
);
/// NVML (`nvml-sys`).
pub const NVML: CatalogEntry = entry(
    "nvml",
    Group::Driver,
    &["nvml.dll"],
    &["libnvidia-ml.so.1"],
);
/// NVDEC / CUVID (`nvcuvid-sys`).
pub const NVCUVID: CatalogEntry = entry(
    "nvcuvid",
    Group::Driver,
    &["nvcuvid.dll"],
    &["libnvcuvid.so.1"],
);
/// NVENC (`nvenc-sys`).
pub const NVENC: CatalogEntry = entry(
    "nvenc",
    Group::Driver,
    &["nvEncodeAPI64.dll"],
    &["libnvidia-encode.so.1"],
);
/// NVIDIA Optical Flow (`nvofa-sys`).
pub const NVOFA: CatalogEntry = entry(
    "nvofa",
    Group::Driver,
    &["nvofapi64.dll"],
    &["libnvidia-opticalflow.so.1"],
);
/// NVAPI, Windows only (`nvapi-sys`).
pub const NVAPI: CatalogEntry = entry("nvapi", Group::Driver, &["nvapi64.dll"], &[]);
/// OptiX, inside the driver's DriverStore folder (`optix-sys`).
pub const OPTIX: CatalogEntry = entry(
    "optix",
    Group::DriverStore,
    &["nvoptix.dll"],
    &["libnvoptix.so.1"],
);

// CUDA Toolkit.

/// cuBLAS (`cublas-sys`).
pub const CUBLAS: CatalogEntry = entry(
    "cublas",
    Group::Toolkit,
    &["cublas64_*"],
    &["libcublas.so.13", "libcublas.so"],
);
/// cuBLASLt (`cublaslt-sys`).
pub const CUBLASLT: CatalogEntry = entry(
    "cublaslt",
    Group::Toolkit,
    &["cublasLt64_*"],
    &["libcublasLt.so.13", "libcublasLt.so"],
);
/// cuFFT (`cufft-sys`).
pub const CUFFT: CatalogEntry = entry(
    "cufft",
    Group::Toolkit,
    &["cufft64_*"],
    &["libcufft.so.12", "libcufft.so"],
);
/// cuFile / GPUDirect Storage, Linux only (`cufile-sys`).
pub const CUFILE: CatalogEntry = entry(
    "cufile",
    Group::Toolkit,
    &[],
    &["libcufile.so.0", "libcufile.so"],
);
/// CUPTI (`cupti-sys`).
pub const CUPTI: CatalogEntry = entry(
    "cupti",
    Group::Toolkit,
    &["cupti64_*"],
    &["libcupti.so.13", "libcupti.so.12", "libcupti.so"],
);
/// cuRAND (loaded by `cuda-bindings`; probed by diagnostics).
pub const CURAND: CatalogEntry = entry(
    "curand",
    Group::Toolkit,
    &["curand64_*"],
    &["libcurand.so.10", "libcurand.so"],
);
/// cuSOLVER (`cusolver-sys`).
pub const CUSOLVER: CatalogEntry = entry(
    "cusolver",
    Group::Toolkit,
    &["cusolver64_*"],
    &["libcusolver.so.12", "libcusolver.so"],
);
/// cuSOLVERMg (`cusolvermg-sys`).
pub const CUSOLVERMG: CatalogEntry = entry(
    "cusolvermg",
    Group::Toolkit,
    &["cusolverMg64_*"],
    &["libcusolverMg.so.12", "libcusolverMg.so"],
);
/// cuSPARSE (`cusparse-sys`).
pub const CUSPARSE: CatalogEntry = entry(
    "cusparse",
    Group::Toolkit,
    &["cusparse64_*"],
    &["libcusparse.so.12", "libcusparse.so"],
);
/// nvJPEG (`nvjpeg-sys`).
pub const NVJPEG: CatalogEntry = entry(
    "nvjpeg",
    Group::Toolkit,
    &["nvjpeg64_*"],
    &["libnvjpeg.so.13", "libnvjpeg.so.12", "libnvjpeg.so"],
);
/// NVRTC (`nvrtc-sys`).
pub const NVRTC: CatalogEntry = entry(
    "nvrtc",
    Group::Toolkit,
    &["nvrtc64_*"],
    &["libnvrtc.so.13", "libnvrtc.so"],
);
/// NPP core (`npp-sys::nppc`).
pub const NPPC: CatalogEntry = entry(
    "nppc",
    Group::Toolkit,
    &["nppc64_*"],
    &["libnppc.so.13", "libnppc.so"],
);
/// NPP signal processing (`npp-sys::npps`).
pub const NPPS: CatalogEntry = entry(
    "npps",
    Group::Toolkit,
    &["npps64_*"],
    &["libnpps.so.13", "libnpps.so"],
);
/// NPP image support functions (`npp-sys::nppisu`).
pub const NPPISU: CatalogEntry = entry(
    "nppisu",
    Group::Toolkit,
    &["nppisu64_*"],
    &["libnppisu.so.13", "libnppisu.so"],
);
/// NPP image data exchange and initialisation (`npp-sys::nppidei`).
pub const NPPIDEI: CatalogEntry = entry(
    "nppidei",
    Group::Toolkit,
    &["nppidei64_*"],
    &["libnppidei.so.13", "libnppidei.so"],
);
/// NPP image arithmetic and logical operations (`npp-sys::nppial`).
pub const NPPIAL: CatalogEntry = entry(
    "nppial",
    Group::Toolkit,
    &["nppial64_*"],
    &["libnppial.so.13", "libnppial.so"],
);
/// NPP image colour conversion (`npp-sys::nppicc`).
pub const NPPICC: CatalogEntry = entry(
    "nppicc",
    Group::Toolkit,
    &["nppicc64_*"],
    &["libnppicc.so.13", "libnppicc.so"],
);
/// NPP image threshold and compare (`npp-sys::nppitc`).
pub const NPPITC: CatalogEntry = entry(
    "nppitc",
    Group::Toolkit,
    &["nppitc64_*"],
    &["libnppitc.so.13", "libnppitc.so"],
);
/// NPP image morphology (`npp-sys::nppim`).
pub const NPPIM: CatalogEntry = entry(
    "nppim",
    Group::Toolkit,
    &["nppim64_*"],
    &["libnppim.so.13", "libnppim.so"],
);
/// NPP image filtering (`npp-sys::nppif`).
pub const NPPIF: CatalogEntry = entry(
    "nppif",
    Group::Toolkit,
    &["nppif64_*"],
    &["libnppif.so.13", "libnppif.so"],
);
/// NPP image statistics and linear transforms (`npp-sys::nppist`).
pub const NPPIST: CatalogEntry = entry(
    "nppist",
    Group::Toolkit,
    &["nppist64_*"],
    &["libnppist.so.13", "libnppist.so"],
);
/// NPP image geometry transforms (`npp-sys::nppig`).
pub const NPPIG: CatalogEntry = entry(
    "nppig",
    Group::Toolkit,
    &["nppig64_*"],
    &["libnppig.so.13", "libnppig.so"],
);

// Separately installed SDKs.

/// cuDNN (`cudnn-sys`).
pub const CUDNN: CatalogEntry = entry(
    "cudnn",
    sdk("CUDNN_ROOT"),
    &["cudnn64_*"],
    &["libcudnn.so.9", "libcudnn.so"],
);
/// nvCOMP (`nvcomp-sys`).
pub const NVCOMP: CatalogEntry = entry(
    "nvcomp",
    sdk("NVCOMP_ROOT"),
    &["nvcomp64_*"],
    &["libnvcomp.so.6", "libnvcomp.so"],
);
/// NCCL, Linux only (`nccl-sys`).
pub const NCCL: CatalogEntry = entry(
    "nccl",
    sdk("NCCL_ROOT"),
    &[],
    &["libnccl.so.2", "libnccl.so"],
);
/// DLSS Super Resolution runtime of the NGX SDK (no sys crate loads it: NGX loads it itself;
/// staged next to applications by `xtask stage-dlls`). The Linux runtime has a versioned file name
/// that is not known here, so no Linux name is listed.
pub const DLSS: CatalogEntry = entry("dlss", sdk("NGX_SDK_ROOT"), &["nvngx_dlss.dll"], &[]);
/// DLSS Ray Reconstruction runtime of the NGX SDK (see [`DLSS`]).
pub const DLSSD: CatalogEntry = entry("dlssd", sdk("NGX_SDK_ROOT"), &["nvngx_dlssd.dll"], &[]);

/// Every library in the catalog: driver, then toolkit, then SDK libraries.
pub const ALL: &[CatalogEntry] = &[
    NVCUDA, NVML, NVCUVID, NVENC, NVOFA, NVAPI, OPTIX, CUBLAS, CUBLASLT, CUFFT, CUFILE, CUPTI,
    CURAND, CUSOLVER, CUSOLVERMG, CUSPARSE, NVJPEG, NVRTC, NPPC, NPPS, NPPISU, NPPIDEI, NPPIAL,
    NPPICC, NPPITC, NPPIM, NPPIF, NPPIST, NPPIG, CUDNN, NVCOMP, NCCL, DLSS, DLSSD,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_unique_and_match_the_spec() {
        let mut names: Vec<&str> = ALL.iter().map(|e| e.name).collect();
        for e in ALL {
            assert_eq!(e.name, e.spec.lib, "catalog name and spec.lib must agree");
            assert!(
                !e.spec.windows.is_empty() || !e.spec.linux.is_empty(),
                "{} names no file on any OS",
                e.name
            );
        }
        names.sort_unstable();
        let before = names.len();
        names.dedup();
        assert_eq!(before, names.len(), "duplicate catalog names");
    }

    #[test]
    fn catalog_sdk_entries_carry_no_build_machine_path() {
        for e in ALL {
            if let Group::Sdk { hint, .. } = e.spec.group {
                assert_eq!(hint, None, "{}: the catalog must not embed a hint", e.name);
            }
        }
    }

    #[test]
    fn linux_names_are_sonames_not_patterns() {
        for e in ALL {
            for name in e.spec.linux {
                assert!(!name.contains('*'), "{}: {name} is not a soname", e.name);
            }
        }
    }
}
