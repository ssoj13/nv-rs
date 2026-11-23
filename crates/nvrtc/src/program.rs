//! [`Program`]: owns an `nvrtcProgram`, compiles CUDA C++, and extracts PTX/CUBIN/logs/names.
//!
//! **What.** Wraps `nvrtcCreateProgram`, `nvrtcAddNameExpression`, `nvrtcCompileProgram`,
//! `nvrtcGetProgramLogSize`/`nvrtcGetProgramLog`, `nvrtcGetPTXSize`/`nvrtcGetPTX`,
//! `nvrtcGetCUBINSize`/`nvrtcGetCUBIN`, `nvrtcGetLTOIRSize`/`nvrtcGetLTOIR`,
//! `nvrtcGetOptiXIRSize`/`nvrtcGetOptiXIR`, `nvrtcGetLoweredName`, and `nvrtcDestroyProgram`.
//! **Ownership.** `Program` destroys its handle in `Drop` and is `!Sync` (single-owner compilation
//! unit; `Send` is safe because NVRTC programs are independent handles).

use crate::{CompileOptions, Error, NvrtcStatus, Result, check, ensure_loaded, sys};
use std::ffi::{CStr, CString, c_char, c_int};

/// An NVRTC compilation unit (`nvrtcProgram`).
pub struct Program {
    raw: sys::nvrtcProgram,
}

// SAFETY: an `nvrtcProgram` is an independent compiler handle with no thread-local affinity.
unsafe impl Send for Program {}

impl Program {
    /// Creates a new program from CUDA C++ `src`, optional `name` (defaults to
    /// `"default_program"` in NVRTC), and virtual header `(include_name, header_source)` pairs.
    pub fn new(src: &str, name: Option<&str>, headers: &[(&str, &str)]) -> Result<Self> {
        ensure_loaded()?;
        let src_c =
            CString::new(src).map_err(|_| Error::InvalidInput("program source contains NUL"))?;
        let name_c = match name {
            Some(n) => Some(
                CString::new(n).map_err(|_| Error::InvalidInput("program name contains NUL"))?,
            ),
            None => None,
        };
        let mut header_srcs = Vec::with_capacity(headers.len());
        let mut header_names = Vec::with_capacity(headers.len());
        for (inc_name, inc_src) in headers {
            header_names.push(
                CString::new(*inc_name)
                    .map_err(|_| Error::InvalidInput("header name contains NUL"))?,
            );
            header_srcs.push(
                CString::new(*inc_src)
                    .map_err(|_| Error::InvalidInput("header source contains NUL"))?,
            );
        }
        let header_src_ptrs: Vec<*const c_char> = header_srcs.iter().map(|s| s.as_ptr()).collect();
        let header_name_ptrs: Vec<*const c_char> =
            header_names.iter().map(|s| s.as_ptr()).collect();
        let num_headers =
            c_int::try_from(headers.len()).map_err(|_| Error::InvalidInput("too many headers"))?;

        let mut raw: sys::nvrtcProgram = std::ptr::null_mut();
        // SAFETY: `src_c`, `name_c`, and header pointer arrays stay alive for the duration of the
        // `nvrtcCreateProgram` call, and `raw` is a valid out-pointer.
        check("nvrtcCreateProgram", unsafe {
            sys::nvrtcCreateProgram(
                &mut raw,
                src_c.as_ptr(),
                name_c.as_ref().map_or(std::ptr::null(), |s| s.as_ptr()),
                num_headers,
                if header_src_ptrs.is_empty() {
                    std::ptr::null()
                } else {
                    header_src_ptrs.as_ptr()
                },
                if header_name_ptrs.is_empty() {
                    std::ptr::null()
                } else {
                    header_name_ptrs.as_ptr()
                },
            )
        })?;
        Ok(Self { raw })
    }

    /// Registers a `__global__` function or `__device__`/`__constant__` variable name expression
    /// before [`Self::compile`] so its mangled name can be queried afterward with
    /// [`Self::lowered_name`].
    pub fn add_name_expression(&mut self, expr: &str) -> Result<()> {
        let expr_c =
            CString::new(expr).map_err(|_| Error::InvalidInput("name expression contains NUL"))?;
        // SAFETY: `self.raw` is a live `nvrtcProgram` and `expr_c` is NUL-terminated.
        check("nvrtcAddNameExpression", unsafe {
            sys::nvrtcAddNameExpression(self.raw, expr_c.as_ptr())
        })
    }

    /// Compiles the program with `options`.
    ///
    /// Returns the compiler build log on success (which may contain warnings). On failure, returns
    /// [`Error::Compilation`] with the retrieved build log attached. If compilation succeeds but the
    /// log query fails, returns [`Error::Status`] naming `nvrtcGetProgramLogSize` or
    /// `nvrtcGetProgramLog`; the program is still compiled and [`Self::ptx`] remains usable.
    pub fn compile(&mut self, options: &CompileOptions) -> Result<String> {
        let c_opts = options.to_cstrings()?;
        let ptrs: Vec<*const c_char> = c_opts.iter().map(|s| s.as_ptr()).collect();
        let num_opts = c_int::try_from(ptrs.len())
            .map_err(|_| Error::InvalidInput("too many compile options"))?;
        // SAFETY: `self.raw` is live and `ptrs` points to valid NUL-terminated strings for the call.
        let code = unsafe {
            sys::nvrtcCompileProgram(
                self.raw,
                num_opts,
                if ptrs.is_empty() {
                    std::ptr::null()
                } else {
                    ptrs.as_ptr()
                },
            )
        };
        if code == sys::nvrtcResult_NVRTC_SUCCESS {
            self.build_log()
        } else {
            let log = match self.build_log() {
                Ok(l) => l,
                Err(e) => format!("<failed to retrieve build log: {e}>"),
            };
            Err(Error::Compilation {
                code: NvrtcStatus(code),
                log,
            })
        }
    }

    /// Retrieves the build log from the most recent [`Self::compile`] call.
    pub fn build_log(&self) -> Result<String> {
        let mut size = 0usize;
        // SAFETY: `self.raw` is live and `size` is a valid out-pointer.
        check("nvrtcGetProgramLogSize", unsafe {
            sys::nvrtcGetProgramLogSize(self.raw, &mut size)
        })?;
        if size <= 1 {
            return Ok(String::new());
        }
        let mut buf = vec![0u8; size];
        // SAFETY: `buf` has `size` bytes matching `nvrtcGetProgramLogSize`.
        check("nvrtcGetProgramLog", unsafe {
            sys::nvrtcGetProgramLog(self.raw, buf.as_mut_ptr().cast())
        })?;
        Ok(trim_nul_utf8(&buf))
    }

    /// Retrieves the compiled PTX as NUL-terminated bytes (suitable for `cuModuleLoadData`).
    pub fn ptx(&self) -> Result<Vec<u8>> {
        let mut size = 0usize;
        // SAFETY: `self.raw` is live and `size` is a valid out-pointer.
        check("nvrtcGetPTXSize", unsafe {
            sys::nvrtcGetPTXSize(self.raw, &mut size)
        })?;
        if size == 0 {
            return Ok(Vec::new());
        }
        let mut buf = vec![0u8; size];
        // SAFETY: `buf` has `size` bytes matching `nvrtcGetPTXSize`.
        check("nvrtcGetPTX", unsafe {
            sys::nvrtcGetPTX(self.raw, buf.as_mut_ptr().cast())
        })?;
        Ok(buf)
    }

    /// Retrieves the compiled PTX as a UTF-8 `String` (without the trailing NUL).
    pub fn ptx_string(&self) -> Result<String> {
        let bytes = self.ptx()?;
        Ok(trim_nul_utf8(&bytes))
    }

    /// Retrieves the compiled CUBIN bytes, or `None` if NVRTC compiled for a virtual
    /// architecture (`compute_XX`) that does not emit CUBIN.
    pub fn cubin(&self) -> Result<Option<Vec<u8>>> {
        self.read_optional_binary(
            "nvrtcGetCUBINSize",
            // SAFETY: delegates to `nvrtcGetCUBINSize` with live `self.raw`.
            |raw, sz| unsafe { sys::nvrtcGetCUBINSize(raw, sz) },
            "nvrtcGetCUBIN",
            // SAFETY: delegates to `nvrtcGetCUBIN` with sized buffer.
            |raw, ptr| unsafe { sys::nvrtcGetCUBIN(raw, ptr) },
        )
    }

    /// Retrieves the compiled LTO-IR bytes, or `None` if `--dlto` was not enabled.
    pub fn ltoir(&self) -> Result<Option<Vec<u8>>> {
        self.read_optional_binary(
            "nvrtcGetLTOIRSize",
            // SAFETY: delegates to `nvrtcGetLTOIRSize` with live `self.raw`.
            |raw, sz| unsafe { sys::nvrtcGetLTOIRSize(raw, sz) },
            "nvrtcGetLTOIR",
            // SAFETY: delegates to `nvrtcGetLTOIR` with sized buffer.
            |raw, ptr| unsafe { sys::nvrtcGetLTOIR(raw, ptr) },
        )
    }

    /// Retrieves the compiled OptiX-IR bytes, or `None` if OptiX-IR was not produced.
    pub fn optixir(&self) -> Result<Option<Vec<u8>>> {
        self.read_optional_binary(
            "nvrtcGetOptiXIRSize",
            // SAFETY: delegates to `nvrtcGetOptiXIRSize` with live `self.raw`.
            |raw, sz| unsafe { sys::nvrtcGetOptiXIRSize(raw, sz) },
            "nvrtcGetOptiXIR",
            // SAFETY: delegates to `nvrtcGetOptiXIR` with sized buffer.
            |raw, ptr| unsafe { sys::nvrtcGetOptiXIR(raw, ptr) },
        )
    }

    /// Looks up the lowered (mangled) name for an expression previously registered with
    /// [`Self::add_name_expression`].
    pub fn lowered_name(&self, expr: &str) -> Result<String> {
        let expr_c =
            CString::new(expr).map_err(|_| Error::InvalidInput("name expression contains NUL"))?;
        let mut ptr: *const c_char = std::ptr::null();
        // SAFETY: `self.raw` is live, `expr_c` is NUL-terminated, and NVRTC owns the returned
        // string until `nvrtcDestroyProgram` (we copy it immediately).
        check("nvrtcGetLoweredName", unsafe {
            sys::nvrtcGetLoweredName(self.raw, expr_c.as_ptr(), &mut ptr)
        })?;
        if ptr.is_null() {
            return Err(Error::InvalidInput("nvrtcGetLoweredName returned null"));
        }
        // SAFETY: `ptr` is non-null and NUL-terminated, valid while `self` is alive.
        Ok(unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned())
    }

    fn read_optional_binary(
        &self,
        size_call: &'static str,
        get_size: impl FnOnce(sys::nvrtcProgram, *mut usize) -> sys::nvrtcResult,
        data_call: &'static str,
        get_data: impl FnOnce(sys::nvrtcProgram, *mut c_char) -> sys::nvrtcResult,
    ) -> Result<Option<Vec<u8>>> {
        let mut size = 0usize;
        check(size_call, get_size(self.raw, &mut size))?;
        if size == 0 {
            return Ok(None);
        }
        let mut buf = vec![0u8; size];
        check(data_call, get_data(self.raw, buf.as_mut_ptr().cast()))?;
        Ok(Some(buf))
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: `self.raw` was created by `nvrtcCreateProgram` and has not been destroyed.
            let _ = unsafe { sys::nvrtcDestroyProgram(&mut self.raw) };
        }
    }
}

fn trim_nul_utf8(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).trim_end().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_nul_utf8_handles_nul_and_trailing_whitespace() {
        assert_eq!(trim_nul_utf8(b"hello\n\0garbage"), "hello");
        assert_eq!(trim_nul_utf8(b"\0"), "");
        assert_eq!(trim_nul_utf8(b"no nul"), "no nul");
    }

    #[test]
    fn program_new_rejects_interior_nul_without_toolkit() {
        // Note: if toolkit is missing this may return Unavailable before checking NUL,
        // or InvalidInput if NVRTC is present; either way it must be Err.
        let err = Program::new("bad\0src", None, &[]);
        assert!(err.is_err());
    }
}
