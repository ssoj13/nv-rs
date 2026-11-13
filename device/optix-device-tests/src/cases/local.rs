//! Case `no_local`: no function of the embedded PTX keeps a `.local` depot.
//!
//! **Why.** OptiX reports a cuda-oxide depot as the program's *direct* stack (a closest-hit with
//! a 256-byte depot logs "direct stack size 256, continuation stack size 0") and reserves no
//! pipeline stack for it, so the depot aliases whatever local memory the launch has: the
//! `transform` case faulted with `CUDA_ERROR_ILLEGAL_ADDRESS` until `optix_device::transform`
//! stopped interpolating motion keys through a borrowed array (which cuda-oxide placed in a
//! depot). Whether a depot appears depends on how code is written (address-taken arrays,
//! runtime-indexed arrays), so this case scans the PTX every run and names the offenders.

use super::common::{Gpu, R};

/// Runs the case.
pub fn run(g: &Gpu) -> R {
    let ptx = String::from_utf8_lossy(&g.ptx);
    let mut current = "";
    let mut bad = Vec::new();
    for line in ptx.lines() {
        let t = line.trim_start();
        if t.starts_with(".visible .entry") || t.starts_with(".entry") || t.starts_with(".func") || t.starts_with(".visible .func") {
            current = t;
        }
        if t.starts_with(".local") && t.contains("__local_depot") {
            bad.push(format!("{current} -> {t}"));
        }
    }
    if bad.is_empty() { Ok(()) } else { Err(format!("{} function(s) keep a .local depot OptiX does not reserve stack for: {bad:?}", bad.len())) }
}
