//! Case `no_local`: no OptiX program of the embedded PTX keeps a `.local` depot.
//!
//! **Why.** cuda-oxide runs `llc --nvptx-short-ptr`, so a function that needs local memory
//! (address-taken arrays, arrays indexed at run time, loops over arrays it does not unroll) gets
//! a `__local_depot` addressed through 32-bit registers (`cvt.u32.u64` of `%SPL`, then
//! `st.local [%r..]`). OptiX programs fault on that: a hand-written raygen with a 1 KiB depot
//! runs correctly with 64-bit local addresses and raises `CUDA_ERROR_ILLEGAL_ADDRESS` with the
//! same code using 32-bit ones; pipeline stack sizes and the CUDA stack limit do not change it.
//! Small depots can pass by luck (they stay inside memory OptiX did map), which hides the bug
//! until a program grows. Library code therefore indexes arrays with constants only, and this
//! case scans the PTX every run and names any function with a depot.
//!
//! [`BLOCKED`] lists the programs that need a depot by construction and are therefore not
//! launched (each case that owns one fails with [`BLOCKED_WHY`]).

use super::common::{Gpu, R};

/// Programs that need addressable local memory by construction.
pub const BLOCKED: &[&str] = &[
    // optixCoopVec* pointer path (N > 64): OptiX receives the addresses of the operands.
    "__raygen__cv80",
];

/// Why the [`BLOCKED`] programs are not run.
pub const BLOCKED_WHY: &str = "needs a .local depot, and OptiX faults on cuda-oxide's 32-bit (--nvptx-short-ptr) local addressing; fix: 64-bit local pointers in cuda-oxide for OptiX targets";

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
        if t.starts_with(".local") && t.contains("__local_depot") && !BLOCKED.iter().any(|b| current.contains(&format!(" {b}("))) {
            bad.push(format!("{current} -> {t}"));
        }
    }
    if bad.is_empty() { Ok(()) } else { Err(format!("{} function(s) keep a .local depot ({BLOCKED_WHY}): {bad:?}", bad.len())) }
}
