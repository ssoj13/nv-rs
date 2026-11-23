//! [`Subscriber`]: CUPTI callback subscription with `catch_unwind` FFI trampoline and safe
//! `Drop` unsubscribe.
//!
//! **What.** Wraps `cuptiSubscribe`, `cuptiUnsubscribe`, `cuptiEnableDomain`,
//! `cuptiEnableCallback`, and `cuptiGetCallbackName`.
//!
//! **Teardown.** `Drop` unsubscribes, waits for callbacks still running on other threads, drops
//! the closure, and leaks the small shared state block CUPTI was given (one per subscriber),
//! because CUPTI does not document that unsubscribe excludes callbacks already dispatched. A
//! failed `cuptiUnsubscribe` is kept with its real `CUptiResult` in `cuda_core_ext::teardown`
//! (`Library::Cupti`).

use crate::{Error, Result, check, ensure_loaded, panic_payload_to_string, sys};
use cuda_core::Context;
use cuda_core_ext::teardown::{self, Library};
use std::cell::UnsafeCell;
use std::ffi::{CStr, c_void};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
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

/// Callback closure invoked by [`Subscriber`] with `(domain, cbid)`. The raw
/// `CUpti_CallbackData` (API enter/exit site, correlation id, parameters) is not forwarded.
pub type CallbackFn = dyn Fn(CallbackDomain, u32) + Send + Sync;

/// Callback state shared with CUPTI through `userdata`.
///
/// CUPTI does not document that `cuptiUnsubscribe` waits for callbacks already dispatched on
/// other threads, so this block is never freed: a late [`callback_trampoline`] can always
/// dereference it, sees `active == false`, and returns without touching `cb`.
struct SubscriberInner {
    /// Trampoline invocations currently between their `in_flight` increment and decrement.
    in_flight: AtomicUsize,
    /// Cleared by `Subscriber::drop` after `cuptiUnsubscribe`.
    active: AtomicBool,
    /// The user callback; taken (set to `None`) by `Subscriber::drop` once no call can use it.
    cb: UnsafeCell<Option<Box<CallbackFn>>>,
    panic_msg: Mutex<Option<String>>,
}

// SAFETY: `cb` is the only field without its own synchronization. Trampolines read it only after
// incrementing `in_flight` and then observing `active == true`; `Subscriber::drop` takes it only
// after storing `active = false` and then observing `in_flight == 0`. With `SeqCst` on all four
// operations, a trampoline that observed `active == true` is counted in every later `in_flight`
// load of `drop`, so the read and the take never overlap. `CallbackFn` is `Send + Sync`.
unsafe impl Sync for SubscriberInner {}

unsafe extern "C" fn callback_trampoline(
    userdata: *mut c_void,
    domain: sys::CUpti_CallbackDomain,
    cbid: sys::CUpti_CallbackId,
    _cbdata: *const c_void,
) {
    if userdata.is_null() {
        return;
    }
    // SAFETY: `userdata` is the `SubscriberInner` leaked by `Subscriber::new`; it is never freed
    // once CUPTI holds it, so it is valid for any call CUPTI makes, even after unsubscribe.
    let inner = unsafe { &*userdata.cast::<SubscriberInner>() };
    inner.in_flight.fetch_add(1, Ordering::SeqCst);
    if inner.active.load(Ordering::SeqCst) {
        // SAFETY: `active` was observed after this call was counted in `in_flight`, so
        // `Subscriber::drop` has not taken `cb` and will not until the decrement below.
        if let Some(cb) = unsafe { &*inner.cb.get() } {
            let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if let Some(dom) = CallbackDomain::from_sys(domain) {
                    cb(dom, cbid);
                }
            }));
            if let Err(payload) = res {
                let mut slot = inner.panic_msg.lock().unwrap_or_else(|e| e.into_inner());
                slot.get_or_insert_with(|| panic_payload_to_string(payload));
            }
        }
    }
    inner.in_flight.fetch_sub(1, Ordering::SeqCst);
}

/// Owns a `CUpti_SubscriberHandle` and its boxed callback state.
///
/// `Drop` unsubscribes, then waits until no callback is executing before dropping the closure.
/// Dropping a `Subscriber` from inside its own callback therefore never returns.
pub struct Subscriber {
    raw: sys::CUpti_SubscriberHandle,
    inner: *mut SubscriberInner,
    /// Keeps the context the subscription was created under alive until after unsubscribe.
    _cuda: Arc<Context>,
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
        let inner = Box::into_raw(Box::new(SubscriberInner {
            in_flight: AtomicUsize::new(0),
            active: AtomicBool::new(true),
            cb: UnsafeCell::new(Some(callback)),
            panic_msg: Mutex::new(None),
        }));
        let mut raw: sys::CUpti_SubscriberHandle = std::ptr::null_mut();
        // SAFETY: `callback_trampoline` is `extern "C"` with `catch_unwind`, and `inner` stays
        // alive until after `cuptiUnsubscribe` in `Drop`.
        let rc = unsafe {
            sys::cuptiSubscribe(&mut raw, Some(callback_trampoline), inner.cast::<c_void>())
        };
        if rc != sys::CUptiResult_CUPTI_SUCCESS {
            // SAFETY: `cuptiSubscribe` failed so CUPTI does not hold `inner`; reclaim it now.
            drop(unsafe { Box::from_raw(inner) });
            return check("cuptiSubscribe", rc).map(|()| unreachable!());
        }
        Ok(Self {
            raw,
            inner,
            _cuda: cuda.clone(),
        })
    }

    /// Enables or disables all callbacks in `domain` (`cuptiEnableDomain`).
    pub fn enable_domain(&self, domain: CallbackDomain, enable: bool) -> Result<()> {
        // SAFETY: `self.raw` is a live subscriber handle.
        check("cuptiEnableDomain", unsafe {
            sys::cuptiEnableDomain(u32::from(enable), self.raw, domain.to_sys())
        })
    }

    /// Enables or disables a single callback `cbid` in `domain` (`cuptiEnableCallback`).
    pub fn enable_callback(&self, domain: CallbackDomain, cbid: u32, enable: bool) -> Result<()> {
        // SAFETY: `self.raw` is a live subscriber handle.
        check("cuptiEnableCallback", unsafe {
            sys::cuptiEnableCallback(u32::from(enable), self.raw, domain.to_sys(), cbid)
        })
    }

    /// Returns an error if the Rust callback panicked during an earlier CUPTI invocation.
    pub fn check_panic(&self) -> Result<()> {
        // SAFETY: `self.inner` is never freed after a successful `new`, see `SubscriberInner`.
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
            // SAFETY: `self.raw` was created by `cuptiSubscribe` and is unsubscribed once. CUPTI
            // starts no new dispatch to it afterwards; dispatches already under way are handled
            // by the quiescence wait below.
            let rc = unsafe { sys::cuptiUnsubscribe(self.raw) };
            teardown::check(
                Library::Cupti,
                "cuptiUnsubscribe",
                rc as i64,
                sys::CUptiResult_CUPTI_SUCCESS as i64,
            );
        }
        if !self.inner.is_null() {
            // SAFETY: `self.inner` is never freed after a successful `new`, see `SubscriberInner`.
            let inner = unsafe { &*self.inner };
            inner.active.store(false, Ordering::SeqCst);
            while inner.in_flight.load(Ordering::SeqCst) != 0 {
                std::thread::yield_now();
            }
            // SAFETY: `active` is false and no trampoline is counted in `in_flight`, so no
            // trampoline reads `cb` now or later (see the `Sync` proof on `SubscriberInner`).
            let cb = unsafe { (*inner.cb.get()).take() };
            drop(cb);
            // `SubscriberInner` itself is intentionally leaked; see its documentation.
        }
    }
}

/// Looks up the human-readable name of a callback ID (`cuptiGetCallbackName`).
pub fn callback_name(domain: CallbackDomain, cbid: u32) -> Result<String> {
    ensure_loaded()?;
    let mut ptr = std::ptr::null();
    // SAFETY: `ptr` is a valid out-pointer; CUPTI writes a static NUL-terminated string.
    check("cuptiGetCallbackName", unsafe {
        sys::cuptiGetCallbackName(domain.to_sys(), cbid, &mut ptr)
    })?;
    if ptr.is_null() {
        return Ok(String::new());
    }
    // SAFETY: `ptr` is non-null and NUL-terminated.
    Ok(unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned())
}
