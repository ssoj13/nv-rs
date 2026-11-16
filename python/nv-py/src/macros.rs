//! Declarative macros reducing PyO3 submodule registration, enum parsing, and error mapping boilerplate.

/// Creates a Python submodule `nv_rs.<name>`, runs `$body` with `|$m|`, attaches it to
/// `$parent`, and registers it in `sys.modules["nv_rs.<name>"]`.
macro_rules! py_register_submodule {
    ($py:expr, $parent:expr, $name:literal, |$m:ident| $body:block) => {{
        let $m = pyo3::types::PyModule::new($py, $name)?;
        $body
        $parent.add_submodule(&$m)?;
        $py.import("sys")?
            .getattr("modules")?
            .set_item(concat!("nv_rs.", $name), &$m)?;
        Ok::<(), pyo3::PyErr>(())
    }};
}

/// Generates a case-insensitive `fn $fn_name(raw: &str) -> pyo3::PyResult<$ret_ty>` parser
/// that records an `NvInvalidArgumentError` via `crate::errors::record_and_err` on unknown inputs.
macro_rules! py_parse_enum {
    ($fn_name:ident, $category:literal, $op:literal, $ret_ty:ty, $expected:literal, {
        $($pat:pat => $val:expr),+ $(,)?
    }) => {
        fn $fn_name(raw: &str) -> pyo3::PyResult<$ret_ty> {
            match raw.trim().to_ascii_lowercase().as_str() {
                $($pat => Ok($val),)+
                other => Err(crate::errors::record_and_err(
                    $category,
                    $op,
                    format!(concat!("unsupported ", $expected, ": `{}`"), other),
                    crate::errors::ErrorKind::InvalidArgument,
                )),
            }
        }
    };
}

/// Generates a `fn $fn_name(op: &str, err: $err_ty) -> pyo3::PyErr` mapper that classifies
/// native errors into `ErrorKind` and records them via `crate::errors::record_and_err`.
macro_rules! py_err_mapper {
    ($fn_name:ident, $category:literal, $err_ty:ty, {
        $($pat:pat => $kind:expr),+ $(,)?
    }) => {
        fn $fn_name(op: &str, err: $err_ty) -> pyo3::PyErr {
            let kind = match &err {
                $($pat => $kind,)+
            };
            crate::errors::record_and_err($category, op, err, kind)
        }
    };
}
