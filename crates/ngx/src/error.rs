//! NGX error types and `NVSDK_NGX_Result` status mapping.
//!
//! **What.** Defines [`ResultCode`] (an integer newtype over `NVSDK_NGX_Result`), [`Error`]
//! (`thiserror` enum covering NGX API failures, invalid arguments, and unsupported platforms), and
//! the internal [`check`] helper.
//! **Why.** Preserves forward compatibility with newer driver-returned NGX status codes while
//! formatting known `NVSDK_NGX_Result_FAIL_*` codes with readable names and hex values.
//! **How.** Maps the canonical `0xBAD00000`–`0xBAD00012` range in [`ResultCode::name`] without
//! requiring `ngx-sys` on non-Windows targets.
//! **Used by** [`crate::params`] and [`crate::vulkan`].

use std::fmt;

/// Success code (`NVSDK_NGX_Result_Success`).
pub const NGX_RESULT_SUCCESS: i32 = 1;

/// Base failure code (`NVSDK_NGX_Result_Fail = 0xBAD00000`).
const NGX_RESULT_FAIL_BASE: i32 = -1_160_773_632;

/// Bits that classify a result as failure (`NVSDK_NGX_SUCCEED` / `NVSDK_NGX_FAILED`,
/// `nvsdk_ngx_defs.h:183-184`: `(value & 0xFFF00000) == NVSDK_NGX_Result_Fail`).
const NGX_RESULT_CLASS_MASK: u32 = 0xFFF0_0000;

/// Raw `NVSDK_NGX_Result` code wrapped as a `Copy` integer newtype.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ResultCode(pub i32);

impl ResultCode {
    /// Driver or NGX binary is out of date (`NVSDK_NGX_Result_FAIL_OutOfDate`).
    pub const OUT_OF_DATE: Self = Self(NGX_RESULT_FAIL_BASE + 12);

    /// Returns the underlying integer `NVSDK_NGX_Result` value.
    pub const fn raw(self) -> i32 {
        self.0
    }

    /// Returns `true` unless the code is in the `NVSDK_NGX_Result_Fail` (`0xBAD00000`) class.
    ///
    /// Mirrors the SDK predicate `NVSDK_NGX_SUCCEED` (`nvsdk_ngx_defs.h:183`), which the official
    /// helpers use on every call (`NVSDK_NGX_FAILED`, `nvsdk_ngx_helpers.h:97`): any code outside
    /// the failure class counts as success, not only `NVSDK_NGX_Result_Success` (`1`).
    pub const fn is_success(self) -> bool {
        (self.0 as u32 & NGX_RESULT_CLASS_MASK) != NGX_RESULT_FAIL_BASE as u32
    }

    /// Returns the symbolic NGX result name if known.
    pub const fn name(self) -> Option<&'static str> {
        match self.0 {
            1 => Some("NVSDK_NGX_Result_Success"),
            x if x == NGX_RESULT_FAIL_BASE => Some("NVSDK_NGX_Result_Fail"),
            x if x == NGX_RESULT_FAIL_BASE + 1 => Some("NVSDK_NGX_Result_FAIL_FeatureNotSupported"),
            x if x == NGX_RESULT_FAIL_BASE + 2 => Some("NVSDK_NGX_Result_FAIL_PlatformError"),
            x if x == NGX_RESULT_FAIL_BASE + 3 => {
                Some("NVSDK_NGX_Result_FAIL_FeatureAlreadyExists")
            }
            x if x == NGX_RESULT_FAIL_BASE + 4 => Some("NVSDK_NGX_Result_FAIL_FeatureNotFound"),
            x if x == NGX_RESULT_FAIL_BASE + 5 => Some("NVSDK_NGX_Result_FAIL_InvalidParameter"),
            x if x == NGX_RESULT_FAIL_BASE + 6 => {
                Some("NVSDK_NGX_Result_FAIL_ScratchBufferTooSmall")
            }
            x if x == NGX_RESULT_FAIL_BASE + 7 => Some("NVSDK_NGX_Result_FAIL_NotInitialized"),
            x if x == NGX_RESULT_FAIL_BASE + 8 => {
                Some("NVSDK_NGX_Result_FAIL_UnsupportedInputFormat")
            }
            x if x == NGX_RESULT_FAIL_BASE + 9 => Some("NVSDK_NGX_Result_FAIL_RWFlagMissing"),
            x if x == NGX_RESULT_FAIL_BASE + 10 => Some("NVSDK_NGX_Result_FAIL_MissingInput"),
            x if x == NGX_RESULT_FAIL_BASE + 11 => {
                Some("NVSDK_NGX_Result_FAIL_UnableToInitializeFeature")
            }
            x if x == NGX_RESULT_FAIL_BASE + 12 => Some("NVSDK_NGX_Result_FAIL_OutOfDate"),
            x if x == NGX_RESULT_FAIL_BASE + 13 => Some("NVSDK_NGX_Result_FAIL_OutOfGPUMemory"),
            x if x == NGX_RESULT_FAIL_BASE + 14 => Some("NVSDK_NGX_Result_FAIL_UnsupportedFormat"),
            x if x == NGX_RESULT_FAIL_BASE + 15 => {
                Some("NVSDK_NGX_Result_FAIL_UnableToWriteToAppDataPath")
            }
            x if x == NGX_RESULT_FAIL_BASE + 16 => {
                Some("NVSDK_NGX_Result_FAIL_UnsupportedParameter")
            }
            x if x == NGX_RESULT_FAIL_BASE + 17 => Some("NVSDK_NGX_Result_FAIL_Denied"),
            x if x == NGX_RESULT_FAIL_BASE + 18 => Some("NVSDK_NGX_Result_FAIL_NotImplemented"),
            _ => None,
        }
    }
}

impl fmt::Debug for ResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Some(name) => write!(f, "ResultCode({name}, 0x{:08X})", self.0 as u32),
            None => write!(f, "ResultCode({}, 0x{:08X})", self.0, self.0 as u32),
        }
    }
}

impl fmt::Display for ResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Some(name) => write!(f, "{name} (0x{:08X})", self.0 as u32),
            None => write!(f, "NVSDK_NGX_Result({}, 0x{:08X})", self.0, self.0 as u32),
        }
    }
}

/// Everything an NGX call can fail with.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An NGX SDK entry point returned a non-success `NVSDK_NGX_Result`.
    #[error("{call} failed with {code}")]
    Ngx {
        /// The NGX entry point that failed.
        call: &'static str,
        /// The raw `NVSDK_NGX_Result` code.
        code: ResultCode,
    },
    /// Operation is not supported on the current platform or configuration.
    #[error("unsupported: {0}")]
    Unsupported(String),
    /// A caller-supplied argument was invalid before reaching NGX.
    #[error("invalid argument: {0}")]
    InvalidArgument(&'static str),
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Maps a raw `NVSDK_NGX_Result` to [`Result<()>`].
pub fn check(call: &'static str, raw: i32) -> Result<()> {
    let code = ResultCode(raw);
    if code.is_success() {
        Ok(())
    } else {
        Err(Error::Ngx { call, code })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `NVSDK_NGX_SUCCEED(v) ((v & 0xFFF00000) != NVSDK_NGX_Result_Fail)` (`nvsdk_ngx_defs.h:183`).
    fn sdk_succeed(v: u32) -> bool {
        (v & 0xFFF0_0000) != 0xBAD0_0000
    }

    #[test]
    fn result_success_matches_sdk_mask_predicate() {
        for raw in [
            1u32,
            2,
            0,
            0x7FFF_FFFF,
            0xBACF_FFFF,
            0xBAE0_0000,
            0xBAD0_0000,
            0xBAD0_0001,
            0xBAD0_0012,
            0xBAD0_00FF,
            0xBADF_FFFF,
        ] {
            let code = ResultCode(raw as i32);
            assert_eq!(code.is_success(), sdk_succeed(raw), "code 0x{raw:08X}");
            assert_eq!(check("c", raw as i32).is_ok(), sdk_succeed(raw));
        }
        // A non-`1` code outside the failure class is success per the SDK, not an error.
        assert!(ResultCode(2).is_success());
        assert!(!ResultCode::OUT_OF_DATE.is_success());
    }
}
