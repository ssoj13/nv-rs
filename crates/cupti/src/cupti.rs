//! Safe wrapper over `cupti-sys` for CUDA Profiling Tools Interface (CUPTI).
//!
//! **What.**
//! - [`activity`] — [`ActivityProfiler`], [`ActivityKind`], [`ActivityRecord`], and
//!   [`AlignedBufferPool`]: registers `cuptiActivityRegisterCallbacks` with `catch_unwind`
//!   trampolines backed by a reusable 8-byte-aligned buffer pool, enables/disables activity
//!   kinds, flushes records, and drains typed [`ActivityRecord`]s.
//! - [`subscriber`] — [`Subscriber`] and [`CallbackDomain`]: wraps `cuptiSubscribe` /
//!   `cuptiUnsubscribe` with `catch_unwind` callback trampolines, domain/callback enablement,
//!   and callback name lookup.
//! - [`version`] — queries `cuptiGetVersion`.
//!
//! **Note on Events / Metrics / Profiler Target APIs.** In CUDA 13.3, `cupti-sys` binds
//! `cupti.h` (version, result strings, Activity API, and Callback API); legacy `cupti_events.h`
//! / `cupti_metrics.h` and `cupti_profiler_target.h` are separate headers not bound by
//! `cupti-sys`.
//!
//! **Teardown safety.** Both [`ActivityProfiler`] and [`Subscriber`] hold `Arc<cuda_core::Context>`
//! and record any `Drop` teardown error on `Context::record_err` instead of panicking.
//! **Used by** `cupti` GPU tests and profiling tools.

pub mod activity;
pub mod subscriber;

pub use activity::{ActivityKind, ActivityProfiler, ActivityRecord, AlignedBufferPool};
pub use subscriber::{CallbackDomain, CallbackFn, Subscriber};

pub use cupti_sys as sys;

use cuda_core::DriverError;
use std::ffi::CStr;
use std::fmt;

/// Raw `CUptiResult` wrapped in a `Copy` integer newtype so newer toolkit codes stay valid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CuptiStatus(pub u32);

impl CuptiStatus {
    /// `CUPTI_SUCCESS` (`0`).
    pub const SUCCESS: Self = Self(sys::CUptiResult_CUPTI_SUCCESS);

    /// Human-readable description from `cuptiGetResultString`.
    pub fn describe(self) -> &'static str {
        if sys::load().is_err() {
            return "cupti unavailable";
        }
        let mut ptr = std::ptr::null();
        // SAFETY: `cuptiGetResultString` writes a pointer to a static NUL-terminated string.
        let rc = unsafe { sys::cuptiGetResultString(self.0, &mut ptr) };
        if rc != sys::CUptiResult_CUPTI_SUCCESS || ptr.is_null() {
            return "unknown CUptiResult";
        }
        // SAFETY: `ptr` is non-null and points to a static NUL-terminated string from CUPTI.
        unsafe { CStr::from_ptr(ptr) }
            .to_str()
            .unwrap_or("non-utf8 CUptiResult")
    }
}

impl fmt::Display for CuptiStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.describe(), self.0)
    }
}

/// Errors returned by `cupti`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// `cupti` could not be loaded from the CUDA Toolkit (`extras/CUPTI`).
    #[error("CUPTI unavailable: {0}")]
    Unavailable(String),
    /// A CUPTI call returned a non-success status code.
    #[error("{call} failed with {code}")]
    Status {
        /// The CUPTI entry point.
        call: &'static str,
        /// The raw status code.
        code: CuptiStatus,
    },
    /// A CUDA driver call failed.
    #[error("CUDA: {0}")]
    Driver(#[from] DriverError),
    /// An argument failed validation before reaching CUPTI.
    #[error("invalid input: {0}")]
    InvalidInput(&'static str),
    /// A Rust callback panicked inside an FFI trampoline (`catch_unwind`).
    #[error("CUPTI callback panicked: {0}")]
    CallbackPanicked(String),
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn ensure_loaded() -> Result<()> {
    sys::load()
        .map(|_| ())
        .map_err(|e| Error::Unavailable(e.to_string()))
}

pub(crate) fn check(call: &'static str, code: sys::CUptiResult) -> Result<()> {
    if code == sys::CUptiResult_CUPTI_SUCCESS {
        Ok(())
    } else {
        Err(Error::Status {
            call,
            code: CuptiStatus(code),
        })
    }
}

/// Queries the loaded CUPTI API version (`cuptiGetVersion`).
pub fn version() -> Result<u32> {
    ensure_loaded()?;
    let mut ver = 0u32;
    // SAFETY: `ver` is a valid out-pointer.
    check("cuptiGetVersion", unsafe { sys::cuptiGetVersion(&mut ver) })?;
    Ok(ver)
}

pub(crate) fn panic_payload_to_string(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_owned()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "non-string panic payload".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_newtype_is_copy_and_formats() {
        let s = CuptiStatus::SUCCESS;
        let s2 = s;
        assert_eq!(s, s2);
        assert!(s.to_string().contains('0'));
    }
}
