//! [`CompileOptions`]: builder for NVRTC compiler flags passed to `nvrtcCompileProgram`.
//!
//! **What.** Collects target architecture (`--gpu-architecture`), C++ standard (`--std`), macros,
//! include paths, math/debug flags, relocatable device code, and LTO, validating that no option
//! contains an interior NUL byte.
//! **Used by** [`crate::Program::compile`].

use crate::{Error, Result};
use std::ffi::CString;

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

    /// Sets `--fmad=true` or `--fmad=false`.
    pub fn fmad(mut self, enabled: bool) -> Self {
        self.args.push(format!("--fmad={enabled}"));
        self
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
    fn rejects_interior_nul_in_option() {
        let opts = CompileOptions::new().arg("bad\0flag");
        assert!(matches!(opts.to_cstrings(), Err(Error::InvalidInput(_))));
    }
}
