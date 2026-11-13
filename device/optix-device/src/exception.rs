//! Exceptions: throwing user exceptions with up to 8 detail words, and reading the code and
//! details of the exception being handled in an `__exception__*` program.

use crate::raw;

/// Throws a user exception: `code` in `0..=2^30-1`, with up to 8 detail words.
///
/// # Safety
///
/// Valid in every program type except exception programs; the pipeline must enable user
/// exceptions (`OPTIX_EXCEPTION_FLAG_USER`).
#[inline(always)]
pub unsafe fn throw<const N: usize>(code: i32, details: [u32; N]) {
    const { assert!(N <= 8, "at most 8 exception details") };
    let d = {
        let mut d = [0u32; 8];
        let mut i = 0;
        while i < N {
            d[i] = details[i];
            i += 1;
        }
        d
    };
    let c = code as u32;
    // SAFETY: forwarded contract; `N` is a compile-time constant.
    unsafe {
        match N {
            0 => raw::throw_exception_0(c),
            1 => raw::throw_exception_1(c, d[0]),
            2 => raw::throw_exception_2(c, d[0], d[1]),
            3 => raw::throw_exception_3(c, d[0], d[1], d[2]),
            4 => raw::throw_exception_4(c, d[0], d[1], d[2], d[3]),
            5 => raw::throw_exception_5(c, d[0], d[1], d[2], d[3], d[4]),
            6 => raw::throw_exception_6(c, d[0], d[1], d[2], d[3], d[4], d[5]),
            7 => raw::throw_exception_7(c, d[0], d[1], d[2], d[3], d[4], d[5], d[6]),
            _ => raw::throw_exception_8(c, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]),
        }
    }
}

/// The code of the exception being handled (`OPTIX_EXCEPTION_CODE_*` are negative, user codes
/// non-negative).
///
/// # Safety
///
/// Valid in exception programs.
#[inline(always)]
pub unsafe fn code() -> i32 {
    // SAFETY: forwarded contract.
    unsafe { raw::get_exception_code() as i32 }
}

/// Detail word `I` (0..8) of the exception being handled.
///
/// # Safety
///
/// Valid in exception programs.
#[inline(always)]
pub unsafe fn detail<const I: u32>() -> u32 {
    // SAFETY: forwarded contract; `I` is a constant.
    unsafe {
        match I {
            0 => raw::get_exception_detail_0(),
            1 => raw::get_exception_detail_1(),
            2 => raw::get_exception_detail_2(),
            3 => raw::get_exception_detail_3(),
            4 => raw::get_exception_detail_4(),
            5 => raw::get_exception_detail_5(),
            6 => raw::get_exception_detail_6(),
            7 => raw::get_exception_detail_7(),
            _ => const { panic!("OptiX has 8 exception detail words") },
        }
    }
}

/// Address of a NUL-terminated string with the source location of the failing call, when the
/// module was compiled with line info (0 otherwise).
///
/// # Safety
///
/// Valid in exception programs.
#[inline(always)]
pub unsafe fn line_info() -> u64 {
    // SAFETY: forwarded contract.
    unsafe { raw::get_exception_line_info() }
}
