//! [`Subscriber`]: CUPTI callback subscription with `catch_unwind` FFI trampoline and safe
//! `Drop` unsubscribe.
//!
//! **What.** Wraps `cuptiSubscribe`, `cuptiUnsubscribe`, `cuptiEnableDomain`,
//! `cuptiEnableCallback`, and `cuptiGetCallbackName`.

use crate::{Error, Result, check, ensure_loaded, panic_payload_to_string, sys};
use cuda_core::{Context, DriverError};
use std::ffi::{CStr, c_void};
use std::sync::{Arc, Mutex};

/// CUPTI callback domains (`CUpti_CallbackDomain`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CallbackDomain {
    /// CUDA Driver API (`CUPTI_CB_DOMAIN_DRIVER_API`).
    DriverApi,
    /// CUDA Runtime API (`CUPTI_CB_DOMAIN_RUNTIME_API`).
    RuntimeApi,
    /// Resource tracking (`CUPTI_CB_DOMAIN_RESOURCE`).
    Resource,
    /// Synchronization (`CUPTI_CB_DOMAIN_SYNCHRONIZE`).
    Synchronize,
    /// NVTX (`CUPTI_CB_DOMAIN_NVTX`).
    Nvtx,
    /// State (`CUPTI_CB_DOMAIN_STATE`).
    State,
}

impl CallbackDomain {
    pub(crate) fn to_sys(self) -> sys::CUpti_CallbackDomain {
        match self {
            Self::DriverApi => sys::CUpti_CallbackDomain_CUPTI_CB_DOMAIN_DRIVER_API,
            Self::RuntimeApi => sys::CUpti_CallbackDomain_CUPTI_CB_DOMAIN_RUNTIME_API,
            Self::Resource => sys::CUpti_CallbackDomain_CUPTI_CB_DOMAIN_RESOURCE,
            Self::Synchronize => sys::CUpti_CallbackDomain_CUPTI_CB_DOMAIN_SYNCHRONIZE,
            Self::Nvtx => sys::CUpti_CallbackDomain_CUPTI_CB_DOMAIN_NVTX,
            Self::State => sys::CUpti_CallbackDomain_CUPTI_CB_DOMAIN_STATE,
        }
    }

    pub(crate) fn from_sys(raw: sys::CUpti_CallbackDomain) -> Option<Self> {
        match raw {
            sys::CUpti_CallbackDomain_CUPTI_CB_DOMAIN_DRIVER_API => Some(Self::DriverApi),
            sys::CUpti_CallbackDomain_CUPTI_CB_DOMAIN_RUNTIME_API => Some(Self::RuntimeApi),
            sys::CUpti_CallbackDomain_CUPTI_CB_DOMAIN_RESOURCE => Some(Self::Resource),
            sys::CUpti_CallbackDomain_CUPTI_CB_DOMAIN_SYNCHRONIZE => Some(Self::Synchronize),
            sys::CUpti_CallbackDomain_CUPTI_CB_DOMAIN_NVTX => Some(Self::Nvtx),
            sys::CUpti_CallbackDomain_CUPTI_CB_DOMAIN_STATE => Some(Self::State),
            _ => None,
        }
    }
}

/// Callback closure invoked by [`Subscriber`]: `(domain, cbid, raw_cbdata)`.
pub type CallbackFn = dyn Fn(CallbackDomain, u32) + Send + Sync;

struct SubscriberInner {
    cb: Box<CallbackFn>,
    panic_msg: Mutex<Option<String>>,
}

unsafe extern "C" fn callback_trampoline(
    userdata: *mut c_void,
    domain: sys::CUpti_CallbackDomain,
    cbid: sys::CUpti_CallbackId,
    _cbdata: *const c_void,
) {
    if userdata.is_null() {
        return;
    }
    // SAFETY: `userdata` is a `Box<SubscriberInner>` owned by `Subscriber`, alive until after
    // `cuptiUnsubscribe` completes in `Subscriber::drop`.
    let inner = unsafe { &*userdata.cast::<SubscriberInner>() };
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if let Some(dom) = CallbackDomain::from_sys(domain) {
            (inner.cb)(dom, cbid);
        }
    }));
    if let Err(payload) = res {
        let mut slot = inner.panic_msg.lock().unwrap_or_else(|e| e.into_inner());
        slot.get_or_insert_with(|| panic_payload_to_string(payload));
    }
}

/// Owns a `CUpti_SubscriberHandle` and its boxed callback state.
pub struct Subscriber {
    raw: sys::CUpti_SubscriberHandle,
    inner: *mut SubscriberInner,
    cuda: Arc<Context>,
}

// SAFETY: `SubscriberInner` is `Send + Sync` (`CallbackFn: Send + Sync`, `Mutex` for panic slot).
unsafe impl Send for Subscriber {}
// SAFETY: as above.
unsafe impl Sync for Subscriber {}

impl Subscriber {
    /// Subscribes `callback` to CUPTI (`cuptiSubscribe`).
    pub fn new(cuda: &Arc<Context>, callback: Box<CallbackFn>) -> Result<Self> {
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let inner = Box::into_raw(Box::new(SubscriberInner { cb: callback, panic_msg: Mutex::new(None) }));
        let mut raw: sys::CUpti_SubscriberHandle = std::ptr::null_mut();
        // SAFETY: `callback_trampoline` is `extern "C"` with `catch_unwind`, and `inner` stays
        // alive until after `cuptiUnsubscribe` in `Drop`.
        let rc = unsafe { sys::cuptiSubscribe(&mut raw, Some(callback_trampoline), inner.cast::<c_void>()) };
        if rc != sys::CUptiResult_CUPTI_SUCCESS {
            // SAFETY: `cuptiSubscribe` failed so CUPTI does not hold `inner`; reclaim it now.
            drop(unsafe { Box::from_raw(inner) });
            return check("cuptiSubscribe", rc).map(|()| unreachable!());
        }
        Ok(Self { raw, inner, cuda: cuda.clone() })
    }

    /// Enables or disables all callbacks in `domain` (`cuptiEnableDomain`).
    pub fn enable_domain(&self, domain: CallbackDomain, enable: bool) -> Result<()> {
        // SAFETY: `self.raw` is a live subscriber handle.
        check("cuptiEnableDomain", unsafe { sys::cuptiEnableDomain(u32::from(enable), self.raw, domain.to_sys()) })
    }

    /// Enables or disables a single callback `cbid` in `domain` (`cuptiEnableCallback`).
    pub fn enable_callback(&self, domain: CallbackDomain, cbid: u32, enable: bool) -> Result<()> {
        // SAFETY: `self.raw` is a live subscriber handle.
        check("cuptiEnableCallback", unsafe { sys::cuptiEnableCallback(u32::from(enable), self.raw, domain.to_sys(), cbid) })
    }

    /// Returns an error if the Rust callback panicked during an earlier CUPTI invocation.
    pub fn check_panic(&self) -> Result<()> {
        // SAFETY: `self.inner` is alive for the lifetime of `self`.
        let inner = unsafe { &*self.inner };
        let mut slot = inner.panic_msg.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(msg) = slot.take() {
            return Err(Error::CallbackPanicked(msg));
        }
        Ok(())
    }
}

impl Drop for Subscriber {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: `self.raw` was created by `cuptiSubscribe`; unsubscribing first guarantees
            // CUPTI will not invoke `callback_trampoline` after this call returns.
            let rc = unsafe { sys::cuptiUnsubscribe(self.raw) };
            if rc != sys::CUptiResult_CUPTI_SUCCESS {
                self.cuda.record_err(Err::<(), _>(DriverError(cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN)));
            }
        }
        if !self.inner.is_null() {
            // SAFETY: `cuptiUnsubscribe` has completed, so `self.inner` is no longer aliased.
            drop(unsafe { Box::from_raw(self.inner) });
        }
    }
}

/// Looks up the human-readable name of a callback ID (`cuptiGetCallbackName`).
pub fn callback_name(domain: CallbackDomain, cbid: u32) -> Result<String> {
    ensure_loaded()?;
    let mut ptr = std::ptr::null();
    // SAFETY: `ptr` is a valid out-pointer; CUPTI writes a static NUL-terminated string.
    check("cuptiGetCallbackName", unsafe { sys::cuptiGetCallbackName(domain.to_sys(), cbid, &mut ptr) })?;
    if ptr.is_null() {
        return Ok(String::new());
    }
    // SAFETY: `ptr` is non-null and NUL-terminated.
    Ok(unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned())
}
