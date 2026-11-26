//! [`CompileOptions`]: builder for NVRTC compiler flags passed to `nvrtcCompileProgram`.
//!
//! **What.** Collects target architecture (`--gpu-architecture`), C++ standard (`--std`), macros,
//! include paths, math/debug flags, relocatable device code, and LTO, validating that no option
//! contains an interior NUL byte.
//! **Used by** [`crate::Program::compile`].

use crate::{Error, Result};
use std::ffi::CString;

/// Floating-point math precision mode for NVRTC compilation.
///
/// Why: interactive rendering and simulation benefit from `--use_fast_math`, whereas certified
/// hard-Boolean verification and polynomial root-finding (e.g. quartic `Torus` / cubic
/// `LatheCurve` intersections, `gracefold-rs/crates/csg-fit`) require IEEE-754 subnormal
/// preservation and exact division/square-root rounding without FMA contraction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum MathMode {
    /// Enables `--use_fast_math` (flush-to-zero, approximate div/sqrt, FMA enabled).
    #[default]
    Fast,
    /// Strict IEEE-754 single-precision math (`--ftz=false`, `--prec-div=true`,
    /// `--prec-sqrt=true`, `--fmad=false`; never passes `--use_fast_math`).
    StrictF32,
}

/// Compiler options passed to [`crate::Program::compile`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CompileOptions {
    args: Vec<String>,
}

impl CompileOptions {
    /// Creates an empty option set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets `--gpu-architecture=<arch>` (e.g. `"compute_86"` or `"sm_86"`).
    pub fn arch(mut self, arch: &str) -> Self {
        self.args.push(format!("--gpu-architecture={arch}"));
        self
    }

    /// Sets `--std=<std>` (e.g. `"c++17"` or `"c++20"`).
    pub fn std(mut self, std_ver: &str) -> Self {
        self.args.push(format!("--std={std_ver}"));
        self
    }

    /// Adds `-D<name>` or `-D<name>=<value>`.
    pub fn define(mut self, name: &str, value: Option<&str>) -> Self {
        match value {
            Some(v) => self.args.push(format!("--define-macro={name}={v}")),
            None => self.args.push(format!("--define-macro={name}")),
        }
        self
    }

    /// Adds `--include-path=<dir>`.
    pub fn include_dir(mut self, dir: &str) -> Self {
        self.args.push(format!("--include-path={dir}"));
        self
    }

    /// Enables `--use_fast_math`.
    pub fn fast_math(mut self) -> Self {
        self.args.push("--use_fast_math".to_owned());
        self
    }

    /// Sets `--ftz=true` or `--ftz=false` (flush denormal/subnormal floats to zero).
    pub fn ftz(mut self, enabled: bool) -> Self {
        self.args.push(format!("--ftz={enabled}"));
        self
    }

    /// Sets `--prec-div=true` or `--prec-div=false` (IEEE-754 single-precision division).
    pub fn prec_div(mut self, enabled: bool) -> Self {
        self.args.push(format!("--prec-div={enabled}"));
        self
    }

    /// Sets `--prec-sqrt=true` or `--prec-sqrt=false` (IEEE-754 single-precision square root).
    pub fn prec_sqrt(mut self, enabled: bool) -> Self {
        self.args.push(format!("--prec-sqrt={enabled}"));
        self
    }

    /// Sets `--fmad=true` or `--fmad=false`.
    pub fn fmad(mut self, enabled: bool) -> Self {
        self.args.push(format!("--fmad={enabled}"));
        self
    }

    /// Configures strict IEEE-754 single-precision math (`--ftz=false`, `--prec-div=true`,
    /// `--prec-sqrt=true`, `--fmad=false`) for deterministic root-finding and verification
    /// kernels without `--use_fast_math`.
    pub fn strict_f32(self) -> Self {
        self.ftz(false).prec_div(true).prec_sqrt(true).fmad(false)
    }

    /// Applies [`MathMode::Fast`] ([`Self::fast_math`]) or [`MathMode::StrictF32`]
    /// ([`Self::strict_f32`]).
    pub fn math_mode(self, mode: MathMode) -> Self {
        match mode {
            MathMode::Fast => self.fast_math(),
            MathMode::StrictF32 => self.strict_f32(),
        }
    }

    /// Enables `--device-debug` (`-G`).
    pub fn device_debug(mut self) -> Self {
        self.args.push("--device-debug".to_owned());
        self
    }

    /// Enables `--generate-line-info` (`-lineinfo`).
    pub fn line_info(mut self) -> Self {
        self.args.push("--generate-line-info".to_owned());
        self
    }

    /// Enables `--relocatable-device-code=true` (`-rdc=true`).
    pub fn rdc(mut self, enabled: bool) -> Self {
        self.args
            .push(format!("--relocatable-device-code={enabled}"));
        self
    }

    /// Enables `-dlto` (link-time optimization IR generation; `nvrtc.h` spells it
    /// `--dlink-time-opt` / `-dlto`, and NVRTC 13 rejects `--dlto`).
    pub fn dlto(mut self) -> Self {
        self.args.push("-dlto".to_owned());
        self
    }

    /// Appends an arbitrary raw flag.
    pub fn arg(mut self, flag: impl Into<String>) -> Self {
        self.args.push(flag.into());
        self
    }

    /// Returns the accumulated flags as string slices.
    pub fn args(&self) -> &[String] {
        &self.args
    }

    /// Converts all flags into `CString`s, rejecting interior NUL bytes.
    pub(crate) fn to_cstrings(&self) -> Result<Vec<CString>> {
        self.args
            .iter()
            .map(|s| {
                CString::new(s.as_str())
                    .map_err(|_| Error::InvalidInput("compile option contains NUL"))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_accumulates_expected_flags() {
        let opts = CompileOptions::new()
            .arch("compute_86")
            .std("c++20")
            .define("BLOCK_SIZE", Some("256"))
            .define("NDEBUG", None)
            .include_dir("/opt/include")
            .fast_math()
            .fmad(false)
            .line_info()
            .rdc(true)
            .dlto()
            .arg("--extra-flag");
        assert_eq!(
            opts.args(),
            &[
                "--gpu-architecture=compute_86",
                "--std=c++20",
                "--define-macro=BLOCK_SIZE=256",
                "--define-macro=NDEBUG",
                "--include-path=/opt/include",
                "--use_fast_math",
                "--fmad=false",
                "--generate-line-info",
                "--relocatable-device-code=true",
                "-dlto",
                "--extra-flag",
            ]
        );
        assert_eq!(opts.to_cstrings().expect("valid cstrings").len(), 11);
    }

    #[test]
    fn strict_f32_and_math_mode_emit_ieee754_flags_without_fast_math() {
        let strict = CompileOptions::new()
            .arch("compute_75")
            .std("c++17")
            .math_mode(MathMode::StrictF32);
        assert_eq!(
            strict.args(),
            &[
                "--gpu-architecture=compute_75",
                "--std=c++17",
                "--ftz=false",
                "--prec-div=true",
                "--prec-sqrt=true",
                "--fmad=false",
            ]
        );
        assert!(!strict.args().iter().any(|a| a.contains("fast_math")));

        let fast = CompileOptions::new().math_mode(MathMode::Fast);
        assert_eq!(fast.args(), &["--use_fast_math"]);
        assert_eq!(MathMode::default(), MathMode::Fast);
    }

    #[test]
    fn rejects_interior_nul_in_option() {
        let opts = CompileOptions::new().arg("bad\0flag");
        assert!(matches!(opts.to_cstrings(), Err(Error::InvalidInput(_))));
    }
}
