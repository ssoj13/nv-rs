//! CUDA arrays and mipmapped arrays: the opaque, tiled storage that texture and surface objects
//! read from and write to.
//!
//! **What.** [`Array`] (`cuArray3DCreate`) and [`MipmappedArray`] (`cuMipmappedArrayCreate`, or
//! imported through `ExternalMemory::mapped_mipmapped_array`) with blocking host <-> array copies.
//! [`ArrayDesc`] is the one description of a layout, shared with external-memory imports.
//! **Why.** Linear buffers cannot be sampled with hardware filtering or written as surfaces;
//! render-rs's environment maps and OptiX textures need arrays.
//! **Copies are blocking**: they enqueue on the given stream and wait for it, so the host slice
//! can be borrowed safely. Asynchronous uploads belong in pinned staging buffers (cuda-core).

use crate::{invalid_value, teardown};
use cuda_core::sys;
use cuda_core::{Context, DriverError, IntoResult, Stream};
use std::sync::Arc;

/// The element format of each channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArrayFormat {
    /// 8-bit unsigned.
    U8,
    /// 16-bit unsigned.
    U16,
    /// 32-bit unsigned.
    U32,
    /// 32-bit signed.
    I32,
    /// 16-bit float.
    F16,
    /// 32-bit float.
    F32,
}

impl ArrayFormat {
    /// Bytes per channel.
    pub const fn bytes(self) -> usize {
        match self {
            ArrayFormat::U8 => 1,
            ArrayFormat::U16 | ArrayFormat::F16 => 2,
            ArrayFormat::U32 | ArrayFormat::I32 | ArrayFormat::F32 => 4,
        }
    }

    pub(crate) fn to_sys(self) -> sys::CUarray_format {
        match self {
            ArrayFormat::U8 => sys::CUarray_format_enum_CU_AD_FORMAT_UNSIGNED_INT8,
            ArrayFormat::U16 => sys::CUarray_format_enum_CU_AD_FORMAT_UNSIGNED_INT16,
            ArrayFormat::U32 => sys::CUarray_format_enum_CU_AD_FORMAT_UNSIGNED_INT32,
            ArrayFormat::I32 => sys::CUarray_format_enum_CU_AD_FORMAT_SIGNED_INT32,
            ArrayFormat::F16 => sys::CUarray_format_enum_CU_AD_FORMAT_HALF,
            ArrayFormat::F32 => sys::CUarray_format_enum_CU_AD_FORMAT_FLOAT,
        }
    }
}

/// An array layout. `height == 0` is 1D, `depth == 0` is 1D/2D.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArrayDesc {
    /// Width in elements.
    pub width: usize,
    /// Height in elements (0 for 1D).
    pub height: usize,
    /// Depth in elements (0 for 1D/2D; layers when `layered`).
    pub depth: usize,
    /// Channel format.
    pub format: ArrayFormat,
    /// Channels per element: 1, 2 or 4.
    pub channels: u32,
    /// Allow surface load/store (required for a [`crate::SurfaceObject`]).
    pub surface: bool,
    /// `depth` counts layers of a layered 1D/2D array.
    pub layered: bool,
    /// Six square faces (`depth` = 6, or 6 x layers with `layered`); `width == height`.
    pub cubemap: bool,
    /// Sparse (tiled) storage: no memory until tiles are mapped with [`crate::TilePool`]
    /// (`CUDA_ARRAY3D_SPARSE`; 2D, 2D layered or 3D, see [`crate::sparse_arrays_supported`]).
    pub sparse: bool,
}

impl ArrayDesc {
    /// A 2D layout.
    pub const fn d2(width: usize, height: usize, format: ArrayFormat, channels: u32) -> Self {
        Self {
            width,
            height,
            depth: 0,
            format,
            channels,
            surface: false,
            layered: false,
            cubemap: false,
            sparse: false,
        }
    }

    /// Bytes per element (`format.bytes() * channels`).
    pub const fn element_bytes(&self) -> usize {
        self.format.bytes() * self.channels as usize
    }

    /// Bytes of one tightly packed row.
    pub const fn row_bytes(&self) -> usize {
        self.width * self.element_bytes()
    }

    /// Bytes of the whole tightly packed level 0.
    pub fn total_bytes(&self) -> usize {
        self.row_bytes() * self.height.max(1) * self.depth.max(1)
    }

    fn validate(&self) -> Result<(), DriverError> {
        if self.width == 0 || !matches!(self.channels, 1 | 2 | 4) {
            return Err(invalid_value());
        }
        // A cubemap has square faces and 6 (or 6 x layers) of them, as cuArray3DCreate requires.
        if self.cubemap
            && (self.width != self.height
                || self.depth == 0
                || !self.depth.is_multiple_of(6)
                || (!self.layered && self.depth != 6))
        {
            return Err(invalid_value());
        }
        Ok(())
    }

    pub(crate) fn to_sys(self) -> sys::CUDA_ARRAY3D_DESCRIPTOR {
        let mut flags = 0;
        if self.surface {
            flags |= sys::CUDA_ARRAY3D_SURFACE_LDST;
        }
        if self.cubemap {
            flags |= sys::CUDA_ARRAY3D_CUBEMAP;
        }
        if self.sparse {
            flags |= sys::CUDA_ARRAY3D_SPARSE;
        }
        if self.layered {
            flags |= sys::CUDA_ARRAY3D_LAYERED;
        }
        sys::CUDA_ARRAY3D_DESCRIPTOR {
            Width: self.width,
            Height: self.height,
            Depth: self.depth,
            Format: self.format.to_sys(),
            NumChannels: self.channels,
            Flags: flags,
        }
    }
}

/// A CUDA array: owned, or one level of a [`MipmappedArray`] (which then owns the storage).
pub struct Array {
    raw: sys::CUarray,
    desc: ArrayDesc,
    ctx: Arc<Context>,
    /// `false` for a mipmap level: the mipmapped array frees it.
    owned: bool,
}

// SAFETY: a CUarray is a driver handle usable from any thread with its context current.
unsafe impl Send for Array {}
// SAFETY: as above; shared access only enqueues driver operations.
unsafe impl Sync for Array {}

impl Array {
    /// Allocates an array.
    pub fn new(ctx: &Arc<Context>, desc: ArrayDesc) -> Result<Self, DriverError> {
        desc.validate()?;
        ctx.bind_to_thread()?;
        let d = desc.to_sys();
        let mut raw: sys::CUarray = std::ptr::null_mut();
        // SAFETY: valid descriptor and out-pointer.
        unsafe { sys::cuArray3DCreate_v2(&mut raw, &d) }.result()?;
        Ok(Self {
            raw,
            desc,
            ctx: ctx.clone(),
            owned: true,
        })
    }

    /// The layout.
    pub fn desc(&self) -> &ArrayDesc {
        &self.desc
    }

    /// The raw driver handle.
    pub fn cu_array(&self) -> sys::CUarray {
        self.raw
    }

    /// The owning context.
    pub fn context(&self) -> &Arc<Context> {
        &self.ctx
    }

    /// Copies tightly packed host data into the whole array and waits for `stream`.
    pub fn upload(&self, stream: &Stream, data: &[u8]) -> Result<(), DriverError> {
        if data.len() != self.desc.total_bytes() {
            return Err(invalid_value());
        }
        let mut c = self.copy_params();
        c.srcMemoryType = sys::CUmemorytype_enum_CU_MEMORYTYPE_HOST;
        c.srcHost = data.as_ptr().cast();
        c.srcPitch = self.desc.row_bytes();
        c.srcHeight = self.desc.height.max(1);
        c.dstMemoryType = sys::CUmemorytype_enum_CU_MEMORYTYPE_ARRAY;
        c.dstArray = self.raw;
        self.copy(stream, &c)
    }

    /// Copies the whole array into tightly packed host memory and waits for `stream`.
    pub fn download(&self, stream: &Stream, out: &mut [u8]) -> Result<(), DriverError> {
        if out.len() != self.desc.total_bytes() {
            return Err(invalid_value());
        }
        let mut c = self.copy_params();
        c.srcMemoryType = sys::CUmemorytype_enum_CU_MEMORYTYPE_ARRAY;
        c.srcArray = self.raw;
        c.dstMemoryType = sys::CUmemorytype_enum_CU_MEMORYTYPE_HOST;
        c.dstHost = out.as_mut_ptr().cast();
        c.dstPitch = self.desc.row_bytes();
        c.dstHeight = self.desc.height.max(1);
        self.copy(stream, &c)
    }

    fn copy_params(&self) -> sys::CUDA_MEMCPY3D {
        // SAFETY: an all-zero CUDA_MEMCPY3D is the documented starting point.
        let mut c: sys::CUDA_MEMCPY3D = unsafe { std::mem::zeroed() };
        c.WidthInBytes = self.desc.row_bytes();
        c.Height = self.desc.height.max(1);
        c.Depth = self.desc.depth.max(1);
        c
    }

    fn copy(&self, stream: &Stream, c: &sys::CUDA_MEMCPY3D) -> Result<(), DriverError> {
        self.ctx.bind_to_thread()?;
        // SAFETY: the extents match the array and the host slice (checked by the callers); the
        // host memory stays borrowed until the stream is synchronized below.
        unsafe { sys::cuMemcpy3DAsync_v2(c, stream.cu_stream()) }.result()?;
        stream.synchronize()
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        if !self.owned {
            return;
        }
        let raw = self.raw;
        // SAFETY: owned array, no texture/surface object outlives it (they hold the Array's Arc
        // or borrow it for their lifetime).
        teardown(&self.ctx, || unsafe { sys::cuArrayDestroy(raw) });
    }
}

/// A mipmapped array; its levels are [`Array`]s it owns.
pub struct MipmappedArray {
    raw: sys::CUmipmappedArray,
    desc: ArrayDesc,
    levels: u32,
    ctx: Arc<Context>,
    /// Keeps imported external memory alive while it is mapped.
    _keep: Option<Arc<crate::external::ExternalMemory>>,
}

// SAFETY: a CUmipmappedArray is a driver handle usable from any thread with its context current.
unsafe impl Send for MipmappedArray {}
// SAFETY: as above.
unsafe impl Sync for MipmappedArray {}

impl MipmappedArray {
    /// Allocates a mipmapped array of `levels` levels (level 0 described by `desc`).
    pub fn new(ctx: &Arc<Context>, desc: ArrayDesc, levels: u32) -> Result<Self, DriverError> {
        desc.validate()?;
        if levels == 0 {
            return Err(invalid_value());
        }
        ctx.bind_to_thread()?;
        let d = desc.to_sys();
        let mut raw: sys::CUmipmappedArray = std::ptr::null_mut();
        // SAFETY: valid descriptor and out-pointer.
        unsafe { sys::cuMipmappedArrayCreate(&mut raw, &d, levels) }.result()?;
        Ok(Self {
            raw,
            desc,
            levels,
            ctx: ctx.clone(),
            _keep: None,
        })
    }

    pub(crate) fn from_owned_raw(
        raw: sys::CUmipmappedArray,
        desc: ArrayDesc,
        levels: u32,
        ctx: &Arc<Context>,
        keep: Option<Arc<crate::external::ExternalMemory>>,
    ) -> Self {
        Self {
            raw,
            desc,
            levels,
            ctx: ctx.clone(),
            _keep: keep,
        }
    }

    /// Number of levels.
    pub fn levels(&self) -> u32 {
        self.levels
    }

    /// The raw driver handle.
    pub fn cu_mipmapped_array(&self) -> sys::CUmipmappedArray {
        self.raw
    }

    /// The owning context.
    pub fn context(&self) -> &Arc<Context> {
        &self.ctx
    }

    /// Level `level` as an [`Array`] borrowing this mipmapped array.
    pub fn level(&self, level: u32) -> Result<LevelRef<'_>, DriverError> {
        if level >= self.levels {
            return Err(invalid_value());
        }
        self.ctx.bind_to_thread()?;
        let mut raw: sys::CUarray = std::ptr::null_mut();
        // SAFETY: valid level index and out-pointer.
        unsafe { sys::cuMipmappedArrayGetLevel(&mut raw, self.raw, level) }.result()?;
        let shrink = |v: usize| if v == 0 { 0 } else { (v >> level).max(1) };
        let desc = ArrayDesc {
            width: shrink(self.desc.width),
            height: shrink(self.desc.height),
            depth: if self.desc.layered {
                self.desc.depth
            } else {
                shrink(self.desc.depth)
            },
            ..self.desc
        };
        Ok(LevelRef {
            array: Array {
                raw,
                desc,
                ctx: self.ctx.clone(),
                owned: false,
            },
            _parent: std::marker::PhantomData,
        })
    }
}

impl Drop for MipmappedArray {
    fn drop(&mut self) {
        let raw = self.raw;
        // SAFETY: level views borrow `self`, so none is alive; mapped arrays are destroyed with
        // cuMipmappedArrayDestroy as the driver documents.
        teardown(&self.ctx, || unsafe { sys::cuMipmappedArrayDestroy(raw) });
    }
}

/// One level of a [`MipmappedArray`], valid while the mipmapped array is borrowed.
pub struct LevelRef<'a> {
    array: Array,
    _parent: std::marker::PhantomData<&'a MipmappedArray>,
}

impl std::ops::Deref for LevelRef<'_> {
    type Target = Array;
    fn deref(&self) -> &Array {
        &self.array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_sizes_follow_format_and_channels() {
        let d = ArrayDesc::d2(64, 32, ArrayFormat::F16, 4);
        assert_eq!(d.element_bytes(), 8);
        assert_eq!(d.row_bytes(), 512);
        assert_eq!(d.total_bytes(), 512 * 32);
        assert!(
            ArrayDesc { channels: 3, ..d }.validate().is_err(),
            "CUDA arrays take 1, 2 or 4 channels"
        );
    }
}
