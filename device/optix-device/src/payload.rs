//! Typed ray payloads: a [`Payload`] is any `Pod` value of up to 32 32-bit words, carried in the
//! payload registers of `optixTrace` / hit objects and read or written by the programs it reaches.
//!
//! **How.** A payload is copied word by word into the 32 registers the trace intrinsic takes; the
//! programs read and write individual words with `_optix_get_payload` / `_optix_set_payload`,
//! which OptiX requires to name the register by a compile-time constant. [`get`] / [`set`]
//! expand one call per word with a literal index behind a `WORDS > k` guard that the compiler
//! folds away, so only the words of `P` remain and every index is a constant.

use crate::raw;
use bytemuck::Pod;

/// A value carried in payload registers: `Pod`, a multiple of 4 bytes, at most 128 bytes.
pub trait Payload: Pod {
    /// Number of 32-bit payload registers.
    const WORDS: u32 = {
        assert!(size_of::<Self>().is_multiple_of(4), "payload size must be a multiple of 4 bytes");
        assert!(size_of::<Self>() <= 128, "payload is at most 32 words (128 bytes)");
        (size_of::<Self>() / 4) as u32
    };
}

impl<T: Pod> Payload for T {}

/// The 32 payload registers holding `p` (unused words zero).
#[inline(always)]
pub(crate) fn to_words<P: Payload>(p: &P) -> [u32; 32] {
    let mut w = [0u32; 32];
    let _ = P::WORDS; // const-evaluates the size checks
    // SAFETY: `P` is Pod and at most 128 bytes (checked by `WORDS`); `w` holds 128 bytes.
    unsafe { core::ptr::copy_nonoverlapping((p as *const P).cast::<u8>(), w.as_mut_ptr().cast::<u8>(), size_of::<P>()) };
    w
}

/// `P` rebuilt from the first words of `w`.
#[inline(always)]
pub(crate) fn from_words<P: Payload>(w: &[u32; 32]) -> P {
    let _ = P::WORDS;
    let mut p = P::zeroed();
    // SAFETY: as in `to_words`; every bit pattern is a valid `P` (Pod).
    unsafe { core::ptr::copy_nonoverlapping(w.as_ptr().cast::<u8>(), (&mut p as *mut P).cast::<u8>(), size_of::<P>()) };
    p
}

macro_rules! each_word {
    ($n:expr, $k:ident => $body:expr) => {
        each_word!(@ $n, $k => $body; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)
    };
    (@ $n:expr, $k:ident => $body:expr; $($i:literal)*) => {
        $( if $i < $n { let $k: u32 = $i; $body; } )*
    };
}

/// Reads payload register `I` (any program reached by a trace, except exception programs).
///
/// # Safety
///
/// Valid in intersection, any-hit, closest-hit and miss programs; `I` must be below the payload
/// size the pipeline (or the payload type) declares.
#[inline(always)]
pub unsafe fn word<const I: u32>() -> u32 {
    // SAFETY: forwarded contract.
    unsafe { raw::get_payload(I) }
}

/// Writes payload register `I`.
///
/// # Safety
///
/// As [`word`].
#[inline(always)]
pub unsafe fn set_word<const I: u32>(v: u32) {
    // SAFETY: forwarded contract.
    unsafe { raw::set_payload(I, v) }
}

/// Reads the whole typed payload.
///
/// # Safety
///
/// As [`word`]; the trace must have been issued with a payload of type `P`.
#[inline(always)]
pub unsafe fn get<P: Payload>() -> P {
    let mut w = [0u32; 32];
    // SAFETY: forwarded contract; indices are literals below `P::WORDS`.
    each_word!(P::WORDS, k => w[k as usize] = unsafe { raw::get_payload(k) });
    from_words(&w)
}

/// Writes the whole typed payload.
///
/// # Safety
///
/// As [`get`].
#[inline(always)]
pub unsafe fn set<P: Payload>(p: &P) {
    let w = to_words(p);
    // SAFETY: forwarded contract; indices are literals below `P::WORDS`.
    each_word!(P::WORDS, k => unsafe { raw::set_payload(k, w[k as usize]) });
}

/// Declares which payload types (`OPTIX_PAYLOAD_TYPE_ID_*` bits) this program accepts; call it
/// first in programs of modules compiled with several payload types.
///
/// # Safety
///
/// Valid in intersection, any-hit, closest-hit and miss programs; the bits must name payload
/// types the module declares.
#[inline(always)]
pub unsafe fn set_types(type_mask: u32) {
    // SAFETY: forwarded contract.
    unsafe { raw::set_payload_types(type_mask) }
}
