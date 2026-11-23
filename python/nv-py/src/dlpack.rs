//! Minimal, dependency-free DLPack C ABI and `__cuda_array_interface__` support for zero-copy GPU interop.

use std::ffi::{c_char, c_void};
use std::ptr;

/// DLPack device type (`kDLCUDA = 2`, `kDLCPU = 1`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DLDevice {
    pub device_type: i32,
    pub device_id: i32,
}

/// Only the unit tests build CPU tensors; imports reject every device type except `DL_CUDA`.
#[cfg(test)]
pub const DL_CPU: i32 = 1;
pub const DL_CUDA: i32 = 2;

/// DLPack element data type descriptor.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DLDataType {
    /// Type code: `0 = int`, `1 = uint`, `2 = float`, `4 = bfloat`, `5 = complex`, `6 = bool`.
    pub code: u8,
    /// Number of bits per element (e.g., `8`, `16`, `32`, `64`).
    pub bits: u8,
    /// Vector lanes (`1` for scalar).
    pub lanes: u16,
}

/// DLPack tensor descriptor.
#[repr(C)]
pub struct DLTensor {
    pub data: *mut c_void,
    pub device: DLDevice,
    pub ndim: i32,
    pub dtype: DLDataType,
    pub shape: *mut i64,
    pub strides: *mut i64,
    pub byte_offset: u64,
}

/// Legacy/standard `DLManagedTensor` exported inside a `"dltensor"` `PyCapsule`.
#[repr(C)]
pub struct DLManagedTensor {
    pub dl_tensor: DLTensor,
    pub manager_ctx: *mut c_void,
    pub deleter: Option<unsafe extern "C" fn(*mut DLManagedTensor)>,
}

/// Owned metadata kept alive until the DLPack consumer invokes `deleter`.
pub struct DlPackExportCtx<T> {
    pub _owner: T,
    pub shape: Vec<i64>,
    pub strides: Vec<i64>,
}

/// Capsule name expected by DLPack consumers (`b"dltensor\0"`).
pub const DLTENSOR_CAPSULE_NAME: &[u8] = b"dltensor\0";
/// Capsule name after a consumer has taken ownership (`b"used_dltensor\0"`).
pub const USED_DLTENSOR_CAPSULE_NAME: &[u8] = b"used_dltensor\0";

/// Frees a [`DLManagedTensor`] whose `manager_ctx` holds a `Box<DlPackExportCtx<T>>`.
///
/// # Safety
/// Must only be called once by the DLPack consumer or capsule destructor when `self_` was
/// allocated by [`into_dlpack_managed`].
pub unsafe extern "C" fn dlpack_deleter<T>(self_: *mut DLManagedTensor) {
    if self_.is_null() {
        return;
    }
    // SAFETY: `self_` was allocated via `Box::into_raw` in `into_dlpack_managed`.
    let managed = unsafe { Box::from_raw(self_) };
    if !managed.manager_ctx.is_null() {
        // SAFETY: `manager_ctx` was allocated via `Box::into_raw` as `DlPackExportCtx<T>`.
        let _ = unsafe { Box::from_raw(managed.manager_ctx.cast::<DlPackExportCtx<T>>()) };
    }
}

/// Packages an owner handle + device pointer + shape/dtype into a heap-allocated [`DLManagedTensor`].
pub fn into_dlpack_managed<T>(
    owner: T,
    dev_ptr: u64,
    device_id: i32,
    dtype: DLDataType,
    shape: Vec<i64>,
    strides: Vec<i64>,
) -> *mut DLManagedTensor {
    let mut ctx = Box::new(DlPackExportCtx {
        _owner: owner,
        shape,
        strides,
    });
    let ndim = ctx.shape.len() as i32;
    let shape_ptr = if ctx.shape.is_empty() {
        ptr::null_mut()
    } else {
        ctx.shape.as_mut_ptr()
    };
    let strides_ptr = if ctx.strides.is_empty() {
        ptr::null_mut()
    } else {
        ctx.strides.as_mut_ptr()
    };
    let ctx_ptr = Box::into_raw(ctx).cast::<c_void>();

    let managed = Box::new(DLManagedTensor {
        dl_tensor: DLTensor {
            data: dev_ptr as usize as *mut c_void,
            device: DLDevice {
                device_type: DL_CUDA,
                device_id,
            },
            ndim,
            dtype,
            shape: shape_ptr,
            strides: strides_ptr,
            byte_offset: 0,
        },
        manager_ctx: ctx_ptr,
        deleter: Some(dlpack_deleter::<T>),
    });
    Box::into_raw(managed)
}

/// PyCapsule destructor called if the `"dltensor"` capsule is garbage-collected unconsumed.
///
/// # Safety
/// `capsule` must be a valid `PyCapsule` created for `DLManagedTensor`.
pub unsafe extern "C" fn pycapsule_dlpack_destructor(capsule: *mut pyo3::ffi::PyObject) {
    // SAFETY: Valid capsule pointer passed by CPython GC.
    let name = unsafe { pyo3::ffi::PyCapsule_GetName(capsule) };
    if name.is_null() {
        return;
    }
    // Check if still unconsumed (`"dltensor"`). If renamed to `"used_dltensor"`, consumer owns it.
    // SAFETY: `name` is a NUL-terminated C string from PyCapsule_GetName.
    let cname = unsafe { std::ffi::CStr::from_ptr(name) };
    if cname.to_bytes() == b"dltensor" {
        // SAFETY: Pointer was stored under `"dltensor"`.
        let ptr = unsafe {
            pyo3::ffi::PyCapsule_GetPointer(
                capsule,
                DLTENSOR_CAPSULE_NAME.as_ptr().cast::<c_char>(),
            )
        };
        if !ptr.is_null() {
            let managed = ptr.cast::<DLManagedTensor>();
            // SAFETY: `managed` has valid deleter or null.
            if let Some(del) = unsafe { (*managed).deleter } {
                // SAFETY: Unconsumed DLManagedTensor must be freed by its deleter.
                unsafe { del(managed) };
            }
        }
    }
}

/// Canonical dtype mapping descriptor tying short name, aliases, DLPack descriptor, and NumPy typestr together.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DtypeEntry {
    /// Canonical short name (e.g. `"f32"`, `"u8"`).
    pub canonical: &'static str,
    /// Known aliases accepted by `parse_dtype` (e.g. `&["float32", "<f4"]`).
    pub aliases: &'static [&'static str],
    /// Corresponding DLPack data type descriptor.
    pub dl_type: DLDataType,
    /// NumPy `__cuda_array_interface__` typestr (e.g. `"<f4"`).
    pub numpy_typestr: &'static str,
    /// Size of one element in bytes.
    pub itemsize: usize,
}

/// Authoritative mapping table for supported DLPack / CUDA buffer element data types.
///
/// Serves as the Single Source of Truth (SSOT) for both [`parse_dtype`] and [`format_dl_dtype`],
/// guaranteeing zero drift between parsing and formatting.
pub const DTYPE_TABLE: &[DtypeEntry] = &[
    DtypeEntry {
        canonical: "u8",
        aliases: &["uint8", "|u1"],
        dl_type: DLDataType {
            code: 1,
            bits: 8,
            lanes: 1,
        },
        numpy_typestr: "|u1",
        itemsize: 1,
    },
    DtypeEntry {
        canonical: "i8",
        aliases: &["int8", "|i1"],
        dl_type: DLDataType {
            code: 0,
            bits: 8,
            lanes: 1,
        },
        numpy_typestr: "|i1",
        itemsize: 1,
    },
    DtypeEntry {
        canonical: "u16",
        aliases: &["uint16", "<u2"],
        dl_type: DLDataType {
            code: 1,
            bits: 16,
            lanes: 1,
        },
        numpy_typestr: "<u2",
        itemsize: 2,
    },
    DtypeEntry {
        canonical: "i16",
        aliases: &["int16", "<i2"],
        dl_type: DLDataType {
            code: 0,
            bits: 16,
            lanes: 1,
        },
        numpy_typestr: "<i2",
        itemsize: 2,
    },
    DtypeEntry {
        canonical: "u32",
        aliases: &["uint32", "<u4"],
        dl_type: DLDataType {
            code: 1,
            bits: 32,
            lanes: 1,
        },
        numpy_typestr: "<u4",
        itemsize: 4,
    },
    DtypeEntry {
        canonical: "i32",
        aliases: &["int32", "<i4"],
        dl_type: DLDataType {
            code: 0,
            bits: 32,
            lanes: 1,
        },
        numpy_typestr: "<i4",
        itemsize: 4,
    },
    DtypeEntry {
        canonical: "u64",
        aliases: &["uint64", "<u8"],
        dl_type: DLDataType {
            code: 1,
            bits: 64,
            lanes: 1,
        },
        numpy_typestr: "<u8",
        itemsize: 8,
    },
    DtypeEntry {
        canonical: "i64",
        aliases: &["int64", "<i8"],
        dl_type: DLDataType {
            code: 0,
            bits: 64,
            lanes: 1,
        },
        numpy_typestr: "<i8",
        itemsize: 8,
    },
    DtypeEntry {
        canonical: "f16",
        aliases: &["float16", "<f2"],
        dl_type: DLDataType {
            code: 2,
            bits: 16,
            lanes: 1,
        },
        numpy_typestr: "<f2",
        itemsize: 2,
    },
    DtypeEntry {
        canonical: "bf16",
        aliases: &["bfloat16"],
        dl_type: DLDataType {
            code: 4,
            bits: 16,
            lanes: 1,
        },
        numpy_typestr: "<V2",
        itemsize: 2,
    },
    DtypeEntry {
        canonical: "f32",
        aliases: &["float32", "<f4"],
        dl_type: DLDataType {
            code: 2,
            bits: 32,
            lanes: 1,
        },
        numpy_typestr: "<f4",
        itemsize: 4,
    },
    DtypeEntry {
        canonical: "f64",
        aliases: &["float64", "<f8"],
        dl_type: DLDataType {
            code: 2,
            bits: 64,
            lanes: 1,
        },
        numpy_typestr: "<f8",
        itemsize: 8,
    },
    DtypeEntry {
        canonical: "bool",
        aliases: &["|b1"],
        dl_type: DLDataType {
            code: 6,
            bits: 8,
            lanes: 1,
        },
        numpy_typestr: "|b1",
        itemsize: 1,
    },
];

/// Finds a [`DtypeEntry`] matching `dtype` by canonical name or alias (case-insensitive, trimmed).
pub fn find_dtype_entry(dtype: &str) -> Option<&'static DtypeEntry> {
    let norm = dtype.trim().to_ascii_lowercase();
    DTYPE_TABLE
        .iter()
        .find(|entry| entry.canonical == norm || entry.aliases.iter().any(|&a| a == norm))
}

/// Finds a [`DtypeEntry`] matching a given [`DLDataType`].
pub fn find_dtype_by_dl(dt: DLDataType) -> Option<&'static DtypeEntry> {
    DTYPE_TABLE.iter().find(|entry| entry.dl_type == dt)
}

/// Maps short dtype string (`"u8"`, `"i32"`, `"f16"`, `"f32"`, `"f64"`, ...) to `(DLDataType, numpy typestr, itemsize)`.
pub fn parse_dtype(dtype: &str) -> Result<(DLDataType, &'static str, usize), String> {
    find_dtype_entry(dtype)
        .map(|entry| (entry.dl_type, entry.numpy_typestr, entry.itemsize))
        .ok_or_else(|| format!("unsupported dtype `{dtype}`"))
}

/// Formats a [`DLDataType`] back to our canonical short dtype name.
pub fn format_dl_dtype(dt: DLDataType) -> Result<&'static str, String> {
    find_dtype_by_dl(dt)
        .map(|entry| entry.canonical)
        .ok_or_else(|| {
            format!(
                "unsupported DLDataType(code={}, bits={}, lanes={})",
                dt.code, dt.bits, dt.lanes
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dtype_table_roundtrip_all_canonical_types() {
        for entry in DTYPE_TABLE {
            // Canonical name parses correctly
            let (dl, typestr, itemsize) = parse_dtype(entry.canonical).unwrap();
            assert_eq!(dl, entry.dl_type);
            assert_eq!(typestr, entry.numpy_typestr);
            assert_eq!(itemsize, entry.itemsize);

            // DLDataType formats back to the exact canonical name
            let canonical = format_dl_dtype(entry.dl_type).unwrap();
            assert_eq!(canonical, entry.canonical);
        }
    }

    #[test]
    fn dtype_table_parses_all_aliases() {
        for entry in DTYPE_TABLE {
            for &alias in entry.aliases {
                let (dl, typestr, itemsize) = parse_dtype(alias).unwrap();
                assert_eq!(dl, entry.dl_type);
                assert_eq!(typestr, entry.numpy_typestr);
                assert_eq!(itemsize, entry.itemsize);
            }
        }
    }

    #[test]
    fn dtype_table_handles_whitespace_and_case() {
        let (dl1, _, _) = parse_dtype("  F32  ").unwrap();
        assert_eq!(
            dl1,
            DLDataType {
                code: 2,
                bits: 32,
                lanes: 1
            }
        );

        let (dl2, _, _) = parse_dtype("Float32").unwrap();
        assert_eq!(
            dl2,
            DLDataType {
                code: 2,
                bits: 32,
                lanes: 1
            }
        );

        let (dl3, _, _) = parse_dtype("UINT8").unwrap();
        assert_eq!(
            dl3,
            DLDataType {
                code: 1,
                bits: 8,
                lanes: 1
            }
        );
    }

    #[test]
    fn dtype_table_rejects_unsupported() {
        assert!(parse_dtype("complex64").is_err());
        assert!(parse_dtype("unknown").is_err());
        assert!(
            format_dl_dtype(DLDataType {
                code: 99,
                bits: 32,
                lanes: 1
            })
            .is_err()
        );
    }
}
