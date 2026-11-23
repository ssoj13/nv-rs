//! Safe wrapper over `npp-sys` for NVIDIA Performance Primitives (NPP).
//!
//! **What.**
//! - [`NppContext`] — caches the `NppStreamContext` (stream handle, device ID, SM/thread/shared
//!   memory properties, compute capability, stream flags) for a `cuda_core::Stream` so
//!   stream-ordered operations execute with zero per-call host allocations or driver queries.
//! - [`image`] — [`ImageView`], [`ImageViewMut`], [`Size`], [`Rect`], [`Interpolation`],
//!   [`MaskSize`], [`CmpOp`], ROI/stride validation, color conversion (`rgb_to_gray_8u`,
//!   `rgb_to_ycbcr_8u`, `ycbcr_to_rgb_8u`), resize (`resize_8u_c1`, `resize_8u_c3`,
//!   `resize_8u_c4`, `resize_32f_c3`), filtering (`filter_box_8u_c1`, `filter_box_8u_c3`,
//!   `filter_gauss_8u_c1`, `filter_gauss_8u_c3`), and thresholding (`threshold_val_8u_c1`,
//!   `threshold_val_32f_c1`).
//! - [`signal`] — 1D signal operations (`set_32f`, `add_32f`, `mul_32f`, two-phase
//!   `sum_32f_scratch_bytes` + `sum_32f`).
//! - [`lib_version`] — queries `nppGetLibVersion`.
//!
//! **Used by** `npp` GPU tests and image/signal processing pipelines.

pub mod context;
pub mod image;
pub mod signal;

pub use context::{LibVersion, NppContext, lib_version};
pub use image::{CmpOp, ImageView, ImageViewMut, Interpolation, MaskSize, Rect, Size};

pub use npp_sys as sys;

use cuda_core::DriverError;
use std::fmt;

/// Raw `NppStatus` wrapped in a `Copy` integer newtype (`0` = success, `< 0` = error, `> 0` = warning).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NppStatus(pub i32);

impl NppStatus {
    /// `NPP_SUCCESS` (`0`).
    pub const SUCCESS: Self = Self(0);

    /// Returns `true` if the status code indicates an error (`< 0`).
    pub const fn is_error(self) -> bool {
        self.0 < 0
    }

    /// Returns `true` for the warnings that mean NPP performed no operation and left the
    /// destination unwritten (`NPP_NO_OPERATION_WARNING`, `NPP_WRONG_INTERSECTION_ROI_WARNING`,
    /// `NPP_WRONG_INTERSECTION_QUAD_WARNING`).
    pub const fn is_no_operation(self) -> bool {
        matches!(
            self.0,
            sys::NppStatus_NPP_NO_OPERATION_WARNING
                | sys::NppStatus_NPP_WRONG_INTERSECTION_ROI_WARNING
                | sys::NppStatus_NPP_WRONG_INTERSECTION_QUAD_WARNING
        )
    }
}

impl fmt::Display for NppStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NppStatus({})", self.0)
    }
}

/// Errors returned by `npp`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// One of the NPP DLLs could not be loaded from the CUDA Toolkit.
    #[error("NPP unavailable: {0}")]
    Unavailable(String),
    /// An NPP function returned a negative error status, or a warning meaning no operation was
    /// performed ([`NppStatus::is_no_operation`]).
    #[error("{call} failed with {code}")]
    Status {
        /// The NPP entry point.
        call: &'static str,
        /// The raw status code.
        code: NppStatus,
    },
    /// A CUDA driver call failed.
    #[error("CUDA: {0}")]
    Driver(#[from] DriverError),
    /// Operation scratch requirement exceeds the caller's byte budget.
    #[error("needs {requested} device bytes, budget admits {available}")]
    Budget {
        /// Required scratch bytes.
        requested: usize,
        /// Caller-supplied byte budget.
        available: usize,
    },
    /// An argument, ROI, or stride failed validation before reaching NPP.
    #[error("invalid input: {0}")]
    InvalidInput(&'static str),
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Maps a raw `NppStatus` to [`Error::Status`] when negative or when it reports that no operation
/// was performed; other warnings (`> 0`, e.g. the misaligned-ROI performance hint) succeed.
pub(crate) fn check(call: &'static str, code: sys::NppStatus) -> Result<()> {
    let status = NppStatus(code);
    if status.is_error() || status.is_no_operation() {
        Err(Error::Status { call, code: status })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_newtype_distinguishes_errors_and_warnings() {
        assert!(!NppStatus::SUCCESS.is_error());
        assert!(!NppStatus(1).is_error());
        assert!(NppStatus(-37).is_error());
        assert_eq!(NppStatus(-37).to_string(), "NppStatus(-37)");
    }

    /// NPP-2: "no operation performed" warnings are surfaced; harmless warnings still succeed.
    #[test]
    fn check_surfaces_no_operation_warnings() {
        assert!(check("t", sys::NppStatus_NPP_SUCCESS).is_ok());
        assert!(check("t", sys::NppStatus_NPP_MISALIGNED_DST_ROI_WARNING).is_ok());
        assert!(check("t", sys::NppStatus_NPP_DOUBLE_SIZE_WARNING).is_ok());
        for code in [
            sys::NppStatus_NPP_NO_OPERATION_WARNING,
            sys::NppStatus_NPP_WRONG_INTERSECTION_ROI_WARNING,
            sys::NppStatus_NPP_WRONG_INTERSECTION_QUAD_WARNING,
        ] {
            assert!(matches!(
                check("t", code),
                Err(Error::Status { code: c, .. }) if c == NppStatus(code)
            ));
        }
        assert!(check("t", -2).is_err());
    }
}
