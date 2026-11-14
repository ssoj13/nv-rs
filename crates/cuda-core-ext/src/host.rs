//! Host memory the GPU can reach: page-locking memory the caller already owns, and managed
//! (unified) memory.
//!
//! **What.** [`HostRegistration`] page-locks a borrowed slice (`cuMemHostRegister`) for the length
//! of the borrow, optionally mapping it into the device address space; [`ManagedBuffer`] owns
//! `cuMemAllocManaged` memory readable from host and device.
//! **Why.** `cuda-core`'s pinned buffers allocate their own memory; decoders and file readers
//! (ffmpeg-rs, exr-rs) hand over buffers they already own, which registration pins in place.

use crate::{invalid_value, teardown};
use cuda_core::sys;
use cuda_core::{Context, DeviceCopy, DriverError, IntoResult};
use std::marker::PhantomData;
use std::sync::Arc;

/// A slice page-locked for as long as this value lives.
pub struct HostRegistration<'a, T> {
    ptr: *mut T,
    len: usize,
    device_ptr: Option<sys::CUdeviceptr>,
    ctx: Arc<Context>,
    _borrow: PhantomData<&'a mut [T]>,
}

impl<'a, T: DeviceCopy> HostRegistration<'a, T> {
    /// Page-locks `data`. With `map_to_device`, the memory is also mapped into the device address
    /// space ([`Self::device_ptr`]).
    pub fn new(
        ctx: &Arc<Context>,
        data: &'a mut [T],
        map_to_device: bool,
    ) -> Result<Self, DriverError> {
        let bytes = std::mem::size_of_val(data);
        if bytes == 0 {
            return Err(invalid_value());
        }
        ctx.bind_to_thread()?;
        let mut flags = sys::CU_MEMHOSTREGISTER_PORTABLE;
        if map_to_device {
            flags |= sys::CU_MEMHOSTREGISTER_DEVICEMAP;
        }
        // SAFETY: `data` is a live, exclusively borrowed host range of `bytes` bytes; it stays
        // borrowed (so neither freed nor moved) until `Drop` unregisters it.
        unsafe { sys::cuMemHostRegister_v2(data.as_mut_ptr().cast(), bytes, flags) }.result()?;
        let mut reg = Self {
            ptr: data.as_mut_ptr(),
            len: data.len(),
            device_ptr: None,
            ctx: ctx.clone(),
            _borrow: PhantomData,
        };
        if map_to_device {
            let mut dptr: sys::CUdeviceptr = 0;
            // SAFETY: the range was registered with DEVICEMAP just above.
            unsafe { sys::cuMemHostGetDevicePointer_v2(&mut dptr, reg.ptr.cast(), 0) }.result()?;
            reg.device_ptr = Some(dptr);
        }
        Ok(reg)
    }

    /// The device address of the memory, when registered with `map_to_device`.
    pub fn device_ptr(&self) -> Option<sys::CUdeviceptr> {
        self.device_ptr
    }

    /// The registered slice.
    pub fn as_slice(&self) -> &[T] {
        // SAFETY: `ptr`/`len` come from the exclusive borrow held for `'a`.
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }

    /// The registered slice, mutably.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: as above, and `&mut self` makes this access exclusive.
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<T> Drop for HostRegistration<'_, T> {
    fn drop(&mut self) {
        let ptr = self.ptr;
        // SAFETY: the range was registered by `new`; device work touching it must have completed
        // (the caller's stream contract, as for any host memory handed to CUDA).
        teardown(&self.ctx, || unsafe {
            sys::cuMemHostUnregister(ptr.cast())
        });
    }
}

/// Managed (unified) memory holding `len` values of `T`, accessible from host and device.
///
/// On Windows the host must not touch it while device work using it is in flight (no concurrent
/// managed access); synchronize the stream first.
pub struct ManagedBuffer<T: DeviceCopy> {
    ptr: sys::CUdeviceptr,
    len: usize,
    ctx: Arc<Context>,
    _marker: PhantomData<T>,
}

// SAFETY: the memory is owned exclusively by this value; sending it moves ownership.
unsafe impl<T: DeviceCopy + Send> Send for ManagedBuffer<T> {}
// SAFETY: shared access only exposes `&[T]`.
unsafe impl<T: DeviceCopy + Sync> Sync for ManagedBuffer<T> {}

impl<T: DeviceCopy> ManagedBuffer<T> {
    /// Allocates `len` zeroed values.
    pub fn zeroed(ctx: &Arc<Context>, len: usize) -> Result<Self, DriverError> {
        let bytes = len
            .checked_mul(size_of::<T>())
            .filter(|b| *b > 0)
            .ok_or_else(invalid_value)?;
        ctx.bind_to_thread()?;
        let mut ptr: sys::CUdeviceptr = 0;
        #[cfg(windows)]
        let flags = sys::CUmemAttach_flags_enum_CU_MEM_ATTACH_GLOBAL as u32;
        #[cfg(not(windows))]
        let flags = sys::CUmemAttach_flags_enum_CU_MEM_ATTACH_GLOBAL;
        // SAFETY: valid out-pointer and size; globally attached so every stream may use it.
        unsafe { sys::cuMemAllocManaged(&mut ptr, bytes, flags) }.result()?;
        // SAFETY: freshly allocated managed memory of `bytes` bytes, idle and host-accessible.
        unsafe { std::ptr::write_bytes(ptr as *mut u8, 0, bytes) };
        Ok(Self {
            ptr,
            len,
            ctx: ctx.clone(),
            _marker: PhantomData,
        })
    }

    /// The device address (also the host address: managed memory is unified).
    pub fn cu_deviceptr(&self) -> sys::CUdeviceptr {
        self.ptr
    }

    /// Number of values.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the buffer is empty (never: allocation requires `len > 0`).
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The values, from the host (the caller ensures no device work on it is in flight).
    pub fn as_slice(&self) -> &[T] {
        // SAFETY: unified memory of `len` initialised (zeroed, then written) values of `T: DeviceCopy`.
        unsafe { std::slice::from_raw_parts(self.ptr as *const T, self.len) }
    }

    /// The values, mutably, from the host.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: as above, exclusive through `&mut self`.
        unsafe { std::slice::from_raw_parts_mut(self.ptr as *mut T, self.len) }
    }
}

impl<T: DeviceCopy> Drop for ManagedBuffer<T> {
    fn drop(&mut self) {
        let ptr = self.ptr;
        // SAFETY: owned allocation from cuMemAllocManaged, freed with cuMemFree.
        teardown(&self.ctx, || unsafe { sys::cuMemFree_v2(ptr) });
    }
}
