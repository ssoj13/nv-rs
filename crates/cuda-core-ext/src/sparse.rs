//! Sparse (tiled) CUDA arrays: tile geometry, a tile pool of physical memory, and mapping or
//! unmapping tiles of an array onto it.
//!
//! **What.** [`sparse_arrays_supported`] (`CU_DEVICE_ATTRIBUTE_SPARSE_CUDA_ARRAY_SUPPORTED`),
//! [`Array::sparse_properties`] (`cuArrayGetSparseProperties`), [`TilePool`] (`cuMemCreate` with
//! `CU_MEM_CREATE_USAGE_TILE_POOL`) and [`Array::map_tiles`] / [`Array::unmap_tiles`]
//! (`cuMemMapArrayAsync`) for a level-0 region of a sparse 2D array (an [`crate::ArrayDesc`]
//! with `sparse`).
//! **Why.** Virtual texturing: a texture over a sparse array reports per-fetch residency
//! (`tex2D` with `bool* isResident`, device crate `cuda-tex-device` `*_sparse`), so a renderer
//! can stream in only the tiles it touches.
//! **How.** Offsets and extents are in elements and must be multiples of the tile extent
//! (checked here); the pool offset must be a multiple of the pool's 64 KiB tile size (checked). The
//! map call is stream-ordered; work that fetches the tiles must be ordered after it.
//! **Used by** the sparse texture tests of `optix-device-tests` and texture streaming.

use crate::array::Array;
use crate::{invalid_value, teardown};
use cuda_core::sys;
use cuda_core::{Context, DriverError, IntoResult, Stream};
use std::sync::Arc;

/// Whether `ctx`'s device supports sparse CUDA arrays.
pub fn sparse_arrays_supported(ctx: &Context) -> Result<bool, DriverError> {
    ctx.bind_to_thread()?;
    let mut v = 0;
    // SAFETY: valid attribute and out-pointer; the device belongs to `ctx`.
    unsafe {
        sys::cuDeviceGetAttribute(
            &mut v,
            sys::CUdevice_attribute_enum_CU_DEVICE_ATTRIBUTE_SPARSE_CUDA_ARRAY_SUPPORTED,
            ctx.cu_device(),
        )
    }
    .result()?;
    Ok(v != 0)
}

/// Tile geometry of a sparse array (`CUDA_ARRAY_SPARSE_PROPERTIES`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SparseProperties {
    /// Tile extent in elements: width, height, depth.
    pub tile: [u32; 3],
    /// First mip level of the mip tail.
    pub miptail_first_level: u32,
    /// Bytes of the mip tail.
    pub miptail_size: u64,
    /// `CU_ARRAY_SPARSE_PROPERTIES_*` flags.
    pub flags: u32,
}

impl Array {
    /// The tile geometry of this sparse array.
    pub fn sparse_properties(&self) -> Result<SparseProperties, DriverError> {
        if !self.desc().sparse {
            return Err(invalid_value());
        }
        self.context().bind_to_thread()?;
        // SAFETY: an all-zero struct is a valid out-parameter.
        let mut p: sys::CUDA_ARRAY_SPARSE_PROPERTIES = unsafe { std::mem::zeroed() };
        // SAFETY: owned sparse array, valid out-pointer.
        unsafe { sys::cuArrayGetSparseProperties(&mut p, self.cu_array()) }.result()?;
        Ok(SparseProperties {
            tile: [p.tileExtent.width, p.tileExtent.height, p.tileExtent.depth],
            miptail_first_level: p.miptailFirstLevel,
            miptail_size: p.miptailSize,
            flags: p.flags,
        })
    }

    /// Maps the level-0 region at `origin` (x, y in elements) of `extent` (width, height) onto
    /// `pool` at byte `pool_offset`, ordered on `stream`. Origin and extent must be multiples of
    /// the tile extent and inside the array; the pool range must hold the tiles' bytes.
    pub fn map_tiles(
        &self,
        stream: &Stream,
        origin: [u32; 2],
        extent: [u32; 2],
        pool: &TilePool,
        pool_offset: u64,
    ) -> Result<(), DriverError> {
        let (tiles, p) = self.check_region(origin, extent)?;
        let tile_bytes = u64::from(p.tile[0])
            * u64::from(p.tile[1])
            * u64::from(p.tile[2].max(1))
            * self.desc().element_bytes() as u64;
        if tile_bytes != pool.tile_bytes {
            return Err(invalid_value());
        }
        let bytes = tiles
            .checked_mul(pool.tile_bytes)
            .ok_or_else(invalid_value)?;
        if !pool_offset.is_multiple_of(pool.tile_bytes)
            || pool_offset
                .checked_add(bytes)
                .is_none_or(|end| end > pool.size as u64)
            || !crate::same_context(&pool.ctx, self.context())
        {
            return Err(invalid_value());
        }
        let mut info = self.map_info(origin, extent);
        info.memOperationType = sys::CUmemOperationType_enum_CU_MEM_OPERATION_TYPE_MAP;
        info.memHandle.memHandle = pool.handle;
        info.offset = pool_offset;
        self.map_call(stream, &mut info)
    }

    /// Unmaps the level-0 region at `origin` of `extent`, ordered on `stream` (same alignment
    /// rules as [`Array::map_tiles`]).
    pub fn unmap_tiles(
        &self,
        stream: &Stream,
        origin: [u32; 2],
        extent: [u32; 2],
    ) -> Result<(), DriverError> {
        self.check_region(origin, extent)?;
        let mut info = self.map_info(origin, extent);
        info.memOperationType = sys::CUmemOperationType_enum_CU_MEM_OPERATION_TYPE_UNMAP;
        self.map_call(stream, &mut info)
    }

    /// Validates a tile-aligned level-0 region of a sparse 2D array; returns its tile count and
    /// the tile geometry it was checked against (one driver query per map call).
    fn check_region(
        &self,
        origin: [u32; 2],
        extent: [u32; 2],
    ) -> Result<(u64, SparseProperties), DriverError> {
        let d = self.desc();
        if d.layered || d.cubemap || d.depth != 0 || d.height == 0 {
            return Err(invalid_value());
        }
        let p = self.sparse_properties()?;
        let size = [d.width as u64, d.height as u64];
        let mut tiles = 1u64;
        for a in 0..2 {
            let t = p.tile[a];
            if t == 0
                || extent[a] == 0
                || !origin[a].is_multiple_of(t)
                || !extent[a].is_multiple_of(t)
                || u64::from(origin[a]) + u64::from(extent[a])
                    > size[a].next_multiple_of(u64::from(t))
            {
                return Err(invalid_value());
            }
            tiles *= u64::from(extent[a] / t);
        }
        Ok((tiles, p))
    }

    fn map_info(&self, origin: [u32; 2], extent: [u32; 2]) -> sys::CUarrayMapInfo {
        // SAFETY: all-zero is the documented starting point (flags and reserved must be zero).
        let mut info: sys::CUarrayMapInfo = unsafe { std::mem::zeroed() };
        info.resourceType = sys::CUresourcetype_enum_CU_RESOURCE_TYPE_ARRAY;
        info.resource.array = self.cu_array();
        info.subresourceType =
            sys::CUarraySparseSubresourceType_enum_CU_ARRAY_SPARSE_SUBRESOURCE_TYPE_SPARSE_LEVEL;
        info.subresource.sparseLevel = sys::CUarrayMapInfo_st__bindgen_ty_2__bindgen_ty_1 {
            level: 0,
            layer: 0,
            offsetX: origin[0],
            offsetY: origin[1],
            offsetZ: 0,
            extentWidth: extent[0],
            extentHeight: extent[1],
            extentDepth: 1,
        };
        info.memHandleType = sys::CUmemHandleType_enum_CU_MEM_HANDLE_TYPE_GENERIC;
        info
    }

    fn map_call(&self, stream: &Stream, info: &mut sys::CUarrayMapInfo) -> Result<(), DriverError> {
        let ordinal = u32::try_from(self.context().ordinal()).map_err(|_| invalid_value())?;
        info.deviceBitMask = 1u32.checked_shl(ordinal).ok_or_else(invalid_value)?;
        if !crate::same_context(stream.context(), self.context()) {
            return Err(invalid_value());
        }
        self.context().bind_to_thread()?;
        // SAFETY: one fully initialized map info for an owned sparse array (and a live pool for
        // a map); the stream belongs to the array's context.
        unsafe { sys::cuMemMapArrayAsync(info, 1, stream.cu_stream()) }.result()
    }
}

/// Physical memory that sparse-array tiles are mapped onto (`cuMemCreate` with
/// `CU_MEM_CREATE_USAGE_TILE_POOL`). Released on drop; the driver keeps it alive while mapped.
pub struct TilePool {
    handle: sys::CUmemGenericAllocationHandle,
    size: usize,
    tile_bytes: u64,
    ctx: Arc<Context>,
}

// SAFETY: a generic allocation handle is a driver handle usable from any thread.
unsafe impl Send for TilePool {}
// SAFETY: as above; shared access only maps from it.
unsafe impl Sync for TilePool {}

impl TilePool {
    /// A pool of at least `tiles` tiles of 64 KiB (the sparse tile size of every CUDA format),
    /// rounded up to the device's allocation granularity.
    pub fn new(ctx: &Arc<Context>, tiles: usize) -> Result<Self, DriverError> {
        const TILE: usize = 64 * 1024;
        ctx.bind_to_thread()?;
        // SAFETY: all-zero is the documented starting point.
        let mut prop: sys::CUmemAllocationProp = unsafe { std::mem::zeroed() };
        prop.type_ = sys::CUmemAllocationType_enum_CU_MEM_ALLOCATION_TYPE_PINNED;
        prop.location.type_ = sys::CUmemLocationType_enum_CU_MEM_LOCATION_TYPE_DEVICE;
        cuda_bindings::set_mem_location_id(&mut prop.location, ctx.cu_device());
        prop.allocFlags.usage = sys::CU_MEM_CREATE_USAGE_TILE_POOL as u16;
        let mut gran = 0usize;
        // SAFETY: valid property and out-pointer.
        unsafe {
            sys::cuMemGetAllocationGranularity(
                &mut gran,
                &prop,
                sys::CUmemAllocationGranularity_flags_enum_CU_MEM_ALLOC_GRANULARITY_MINIMUM,
            )
        }
        .result()?;
        let size = tiles
            .checked_mul(TILE)
            .ok_or_else(invalid_value)?
            .next_multiple_of(gran.max(TILE));
        if tiles == 0 {
            return Err(invalid_value());
        }
        let mut handle = 0;
        // SAFETY: valid property and out-pointer; `size` is a multiple of the granularity.
        unsafe { sys::cuMemCreate(&mut handle, size, &prop, 0) }.result()?;
        Ok(Self {
            handle,
            size,
            tile_bytes: TILE as u64,
            ctx: ctx.clone(),
        })
    }

    /// Bytes in the pool.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Bytes per tile (pool offsets are multiples of it).
    pub fn tile_bytes(&self) -> u64 {
        self.tile_bytes
    }
}

impl Drop for TilePool {
    fn drop(&mut self) {
        let h = self.handle;
        // SAFETY: owned handle; the driver defers freeing while tiles are still mapped.
        teardown(&self.ctx, || unsafe { sys::cuMemRelease(h) });
    }
}
