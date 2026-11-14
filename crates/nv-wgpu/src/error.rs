//! Error and result types for `nv-wgpu` CUDA <-> `wgpu` interop.
//!
//! Why: `nv-wgpu` bridges `cuda-core` / `cuda-core-ext`, `wgpu`, `ash` (Vulkan), and
//! Win32/DX12 (`windows` on Windows). Unifying failures into a single typed
//! enum prevents silent backend downgrades and surfaces explicit unsupported
//! extension, format, device-mismatch, or validation diagnostics.
//!
//! Used by every module in `nv-wgpu` and re-exported at the crate root.

use std::fmt;

use cuda_core::DriverError;

/// Crate-wide result alias for `nv-wgpu`.
pub type Result<T> = std::result::Result<T, Error>;

/// Typed error for CUDA <-> `wgpu` 30 external memory and synchronization interop.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error propagated from the CUDA driver via `cuda-core` or `cuda-core-ext`.
    #[error("cuda driver error: {0}")]
    Driver(#[from] DriverError),

    /// Required backend capability, extension, or texture format is not supported.
    ///
    /// `nv-wgpu` never silently downgrades an explicitly requested backend or format.
    #[error("unsupported {what}: required {required}, found {found}")]
    Unsupported {
        /// Subsystem or resource property that lacked support.
        what: &'static str,
        /// Required extension, format, or backend capability.
        required: &'static str,
        /// Actual capability or value observed on the device/request.
        found: String,
    },

    /// `wgpu` device backend does not match the requested or supported interop backend.
    #[error("backend mismatch: expected {expected}, found {found:?}")]
    BackendMismatch {
        /// Backend name required for the operation.
        expected: &'static str,
        /// Actual backend reported by `wgpu`.
        found: wgpu::Backend,
    },

    /// The supplied `wgpu` adapter/device and CUDA context refer to different GPUs.
    #[error("device mismatch between wgpu ({wgpu_id}) and CUDA ({cuda_id})")]
    DeviceMismatch {
        /// Human-readable UUID or LUID of the `wgpu` GPU.
        wgpu_id: String,
        /// Human-readable UUID or LUID of the CUDA GPU.
        cuda_id: String,
    },

    /// No CUDA device matches the `wgpu` adapter UUID/LUID.
    #[error("no CUDA device matches wgpu adapter ({wgpu_id}) across {device_count} CUDA device(s)")]
    NoMatchingCudaDevice {
        /// Human-readable UUID or LUID of the `wgpu` GPU.
        wgpu_id: String,
        /// Total CUDA devices inspected.
        device_count: i32,
    },

    /// Raw Vulkan API call failure.
    #[error("vulkan call `{call}` failed: {code:?}")]
    Vulkan {
        /// Vulkan function or operation name.
        call: &'static str,
        /// Returned `VkResult`.
        code: ash::vk::Result,
    },

    /// Direct3D 12 / DXGI / Win32 API failure (HRESULT code).
    #[cfg(windows)]
    #[error("dx12/win32 call `{call}` failed: HRESULT 0x{hresult:08x} ({message})")]
    Dx12 {
        /// Direct3D 12 or Win32 call name.
        call: &'static str,
        /// Raw `HRESULT` bit pattern.
        hresult: u32,
        /// Error message from `windows::core::Error`.
        message: String,
    },

    /// Caller passed an invalid descriptor, size, alignment, or usage combination.
    #[error("invalid argument: {0}")]
    InvalidInput(&'static str),

    /// OS kernel handle or file descriptor export/ownership failure.
    #[error("invalid external handle from {source_api}")]
    InvalidHandle {
        /// API that produced the invalid handle or descriptor.
        source_api: &'static str,
    },

    /// Failed to request a `wgpu` adapter or device.
    #[error("wgpu request failed: {0}")]
    WgpuRequest(String),
}

impl Error {
    /// Constructs [`Error::Vulkan`] from an `ash::vk::Result`.
    #[inline]
    pub(crate) fn vulkan(call: &'static str, code: ash::vk::Result) -> Self {
        Self::Vulkan { call, code }
    }

    /// Constructs [`Error::Dx12`] from a `windows::core::Error`.
    #[cfg(windows)]
    #[inline]
    pub(crate) fn dx12(call: &'static str, err: windows::core::Error) -> Self {
        Self::Dx12 {
            call,
            hresult: err.code().0 as u32,
            message: err.message(),
        }
    }

    /// Formats a 16-byte device UUID as lowercase hex for diagnostics.
    pub(crate) fn fmt_uuid(uuid: &[u8; 16]) -> String {
        fmt_hex(uuid)
    }

    /// Formats an 8-byte adapter LUID as lowercase hex for diagnostics.
    #[cfg(windows)]
    pub(crate) fn fmt_luid(luid: &[u8; 8]) -> String {
        fmt_hex(luid)
    }
}

/// Formats an arbitrary byte slice as contiguous lowercase hexadecimal.
fn fmt_hex(bytes: &[u8]) -> String {
    use fmt::Write as _;
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        let _ = write!(out, "{b:02x}");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_variants_format_informative_messages() {
        let unsupported = Error::Unsupported {
            what: "texture format",
            required: "CUDA-interoperable uncompressed color format",
            found: "Depth32Float".to_owned(),
        };
        assert!(unsupported.to_string().contains("Depth32Float"));

        let mismatch = Error::DeviceMismatch {
            wgpu_id: Error::fmt_uuid(&[0xab; 16]),
            cuda_id: Error::fmt_uuid(&[0xcd; 16]),
        };
        assert!(mismatch.to_string().contains("abab"));
        assert!(mismatch.to_string().contains("cdcd"));

        let vk_err = Error::vulkan("vkCreateImage", ash::vk::Result::ERROR_OUT_OF_DEVICE_MEMORY);
        assert!(vk_err.to_string().contains("vkCreateImage"));
    }
}
