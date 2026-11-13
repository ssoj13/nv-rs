//! Case `ptr64`: every `.local` and `.const` access of the embedded PTX uses a 64-bit address.
//!
//! **Why.** OptiX programs fault (`CUDA_ERROR_ILLEGAL_ADDRESS`) on state-space addresses held in
//! 32-bit registers, which is what cuda-oxide emits by default (`llc --nvptx-short-ptr`): a
//! `__local_depot` addressed through `cvt.u32.u64` of `%SPL` then `st.local [%r..]`, or a
//! `.const` address picked at run time into a 32-bit register. A hand-written raygen with the
//! same depot runs with 64-bit addresses, and small depots can pass by luck; built with short
//! pointers, case `const_index` and the coop_vec N = 80 pointer path fault. This crate is
//! therefore built with 64-bit pointers in every address space (`CUDA_OXIDE_NO_SHORT_PTR`, set
//! in `.cargo/cuda-oxide.toml`), and this case checks the PTX every run so a build without it
//! fails here, by name, before a launch faults and poisons the context for later cases.
//!
//! The scan needs at least one depot to look at: the coop_vec N = 80 program (pointer path)
//! hands OptiX the addresses of its operands, so it always has one.

use super::common::{Gpu, R};

/// Runs the case.
pub fn run(g: &Gpu) -> R {
    let ptx = String::from_utf8_lossy(&g.ptx);
    let mut current = "";
    let (mut depots, mut accesses) = (0usize, 0usize);
    let mut bad = Vec::new();
    for line in ptx.lines() {
        let t = line.trim_start();
        if t.starts_with(".visible .entry") || t.starts_with(".entry") || t.starts_with(".func") || t.starts_with(".visible .func") {
            current = t;
        }
        if t.starts_with(".local") && t.contains("__local_depot") {
            depots += 1;
        }
        // The depot pointer narrowed to 32 bits: the short-pointer form itself.
        if t.starts_with("cvt.u32.u64") && t.contains("%SP") {
            bad.push(format!("{current} -> {t}"));
        }
        if ["ld.local", "st.local", "ld.const"].iter().any(|op| t.starts_with(op)) {
            accesses += 1;
            if address_is_32_bit(t) {
                bad.push(format!("{current} -> {t}"));
            }
        }
    }
    if !bad.is_empty() {
        return Err(format!("{} state-space access(es) use 32-bit addresses (built without CUDA_OXIDE_NO_SHORT_PTR?): {bad:?}", bad.len()));
    }
    if depots == 0 || accesses == 0 {
        return Err(format!("nothing to check ({depots} depots, {accesses} accesses); the coop_vec N = 80 program should have a depot"));
    }
    println!("ptr64: {depots} depot(s), {accesses} ld/st.local and ld.const, all 64-bit addressed");
    Ok(())
}

/// Whether the `[...]` address of a load or store is a 32-bit register (`%r<n>`; 64-bit ones
/// are `%rd<n>`, `%SP`, `%SPL`, or a symbol).
fn address_is_32_bit(instruction: &str) -> bool {
    let Some(start) = instruction.find('[') else { return false };
    instruction[start + 1..].strip_prefix("%r").is_some_and(|rest| rest.starts_with(|c: char| c.is_ascii_digit()))
}
