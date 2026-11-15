//! Minimal, dependency-free DLPack C ABI and `__cuda_array_interface__` support for zero-copy GPU interop.

use std::ffi::{c_char, c_int, c_void};
use std::ptr;

/// DLPack device type (`kDLCUDA = 2`, `kDLCPU = 1`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DLDevice {
    pub device_type: i32,
    pub device_id: i32,
}

#[allow(dead_code)]
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

/// Maps short dtype string (`"u8"`, `"i32"`, `"f16"`, `"f32"`, `"f64"`, ...) to `(DLDataType, numpy typestr, itemsize)`.
pub fn parse_dtype(dtype: &str) -> Result<(DLDataType, &'static str, usize), String> {
    match dtype.trim().to_ascii_lowercase().as_str() {
        "u8" | "uint8" | "|u1" => Ok((
            DLDataType {
                code: 1,
                bits: 8,
                lanes: 1,
            },
            "|u1",
            1,
        )),
        "i8" | "int8" | "|i1" => Ok((
            DLDataType {
                code: 0,
                bits: 8,
                lanes: 1,
            },
            "|i1",
            1,
        )),
        "u16" | "uint16" | "<u2" => Ok((
            DLDataType {
                code: 1,
                bits: 16,
                lanes: 1,
            },
            "<u2",
            2,
        )),
        "i16" | "int16" | "<i2" => Ok((
            DLDataType {
                code: 0,
                bits: 16,
                lanes: 1,
            },
            "<i2",
            2,
        )),
        "u32" | "uint32" | "<u4" => Ok((
            DLDataType {
                code: 1,
                bits: 32,
                lanes: 1,
            },
            "<u4",
            4,
        )),
        "i32" | "int32" | "<i4" => Ok((
            DLDataType {
                code: 0,
                bits: 32,
                lanes: 1,
            },
            "<i4",
            4,
        )),
        "u64" | "uint64" | "<u8" => Ok((
            DLDataType {
                code: 1,
                bits: 64,
                lanes: 1,
            },
            "<u8",
            8,
        )),
        "i64" | "int64" | "<i8" => Ok((
            DLDataType {
                code: 0,
                bits: 64,
                lanes: 1,
            },
            "<i8",
            8,
        )),
        "f16" | "float16" | "<f2" => Ok((
            DLDataType {
                code: 2,
                bits: 16,
                lanes: 1,
            },
            "<f2",
            2,
        )),
        "bf16" | "bfloat16" => Ok((
            DLDataType {
                code: 4,
                bits: 16,
                lanes: 1,
            },
            "<V2",
            2,
        )),
        "f32" | "float32" | "<f4" => Ok((
            DLDataType {
                code: 2,
                bits: 32,
                lanes: 1,
            },
            "<f4",
            4,
        )),
        "f64" | "float64" | "<f8" => Ok((
            DLDataType {
                code: 2,
                bits: 64,
                lanes: 1,
            },
            "<f8",
            8,
        )),
        "bool" | "|b1" => Ok((
            DLDataType {
                code: 6,
                bits: 8,
                lanes: 1,
            },
            "|b1",
            1,
        )),
        other => Err(format!("unsupported dtype `{other}`")),
    }
}

/// Formats a [`DLDataType`] back to our canonical short dtype name.
pub fn format_dl_dtype(dt: DLDataType) -> Result<&'static str, String> {
    match (dt.code, dt.bits, dt.lanes) {
        (1, 8, 1) => Ok("u8"),
        (0, 8, 1) => Ok("i8"),
        (1, 16, 1) => Ok("u16"),
        (0, 16, 1) => Ok("i16"),
        (1, 32, 1) => Ok("u32"),
        (0, 32, 1) => Ok("i32"),
        (1, 64, 1) => Ok("u64"),
        (0, 64, 1) => Ok("i64"),
        (2, 16, 1) => Ok("f16"),
        (4, 16, 1) => Ok("bf16"),
        (2, 32, 1) => Ok("f32"),
        (2, 64, 1) => Ok("f64"),
        (6, 8, 1) => Ok("bool"),
        _ => Err(format!(
            "unsupported DLDataType(code={}, bits={}, lanes={})",
            dt.code, dt.bits, dt.lanes
        )),
    }
}

/// Extracts `c_int` value for `PyCapsule_SetName` check (unused helper alias).
#[allow(dead_code)]
pub type CapsuleStatus = c_int;
