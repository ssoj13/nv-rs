//! Shared `PyDeviceBuffer` pointer extraction and bounds validation helpers.

use pyo3::prelude::*;

use crate::core::PyDeviceBuffer;
use crate::errors::{ErrorKind, record_and_err};

/// Extracts `(raw_device_ptr, byte_len)` from a `PyDeviceBuffer`, supporting both
/// `nv_rs`-owned buffers and borrowed DLPack tensors.
pub(crate) fn buf_ptr_and_bytes(buf: &Bound<'_, PyDeviceBuffer>) -> (u64, usize) {
    let borrowed = buf.borrow();
    (borrowed.raw_device_ptr(), borrowed.byte_len())
}

/// Validates that `nbytes >= min_bytes` and `ptr != 0` when `min_bytes > 0`.
pub(crate) fn validate_dev_buf(
    category: &str,
    op: &str,
    name: &str,
    ptr: u64,
    nbytes: usize,
    min_bytes: usize,
) -> PyResult<()> {
    if nbytes < min_bytes {
        return Err(record_and_err(
            category,
            op,
            format!("{name} buffer too small: got {nbytes} bytes, need at least {min_bytes} bytes"),
            ErrorKind::InvalidArgument,
        ));
    }
    if min_bytes > 0 && ptr == 0 {
        return Err(record_and_err(
            category,
            op,
            format!("{name} device pointer is null"),
            ErrorKind::InvalidArgument,
        ));
    }
    Ok(())
}
