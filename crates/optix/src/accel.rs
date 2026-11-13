//! Acceleration structures: [`Accel`] over triangle meshes (GAS) or over [`Instance`]s (IAS).
//!
//! **What.** One build call sizes the structure (`optixAccelComputeMemoryUsage`), allocates
//! stream-ordered temp and output buffers, builds on the caller's stream and, with
//! [`BuildOptions::compact`], reads the compacted size back and compacts into a tighter buffer.
//! Input buffers are only read during the build; the result references nothing but its own
//! buffer, except an instance structure, which holds `Arc`s of the structures it instances.
//! **Ordering.** Every buffer is allocated on `stream` and freed with `cuMemFreeAsync` on it, so
//! frees are ordered after the work that uses them; compaction synchronizes `stream` once to
//! read the compacted size.

use crate::{DeviceContext, Error, Result, check, entry, sys};
use cuda_core::{DeviceBuffer, Stream};
use std::sync::Arc;

/// Per-mesh geometry flags (`OptixGeometryFlags`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GeometryFlags(pub u32);

impl GeometryFlags {
    /// Any-hit programs run as usual.
    pub const NONE: Self = Self(sys::OptixGeometryFlags_OPTIX_GEOMETRY_FLAG_NONE as u32);
    /// Opaque geometry: any-hit programs are skipped.
    pub const DISABLE_ANYHIT: Self = Self(sys::OptixGeometryFlags_OPTIX_GEOMETRY_FLAG_DISABLE_ANYHIT as u32);
    /// Any-hit runs at most once per primitive per ray.
    pub const REQUIRE_SINGLE_ANYHIT_CALL: Self = Self(sys::OptixGeometryFlags_OPTIX_GEOMETRY_FLAG_REQUIRE_SINGLE_ANYHIT_CALL as u32);
}

/// One triangle mesh of a geometry structure; it uses one SBT record (hit group) per mesh.
pub struct TriangleMesh<'a> {
    /// Vertex positions.
    pub vertices: &'a DeviceBuffer<[f32; 3]>,
    /// Triangles as vertex-index triplets; `None` means consecutive vertex triplets.
    pub indices: Option<&'a DeviceBuffer<[u32; 3]>>,
    /// Geometry flags.
    pub flags: GeometryFlags,
}

/// How a structure is built.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BuildOptions {
    /// Favour trace speed over build speed.
    pub prefer_fast_trace: bool,
    /// Compact after building (one stream synchronization, usually halves the memory).
    pub compact: bool,
    /// Device bytes the build may allocate at peak (temp + output); `None` admits any size.
    /// Checked after OptiX reports the sizes and before anything is allocated.
    pub budget: Option<usize>,
}

impl Default for BuildOptions {
    /// Fast trace, compacted, no budget: the right choice for static geometry.
    fn default() -> Self {
        Self { prefer_fast_trace: true, compact: true, budget: None }
    }
}

/// One instance of a structure inside an instance structure.
#[derive(Clone)]
pub struct Instance {
    /// The instanced structure.
    pub accel: Arc<Accel>,
    /// Row-major 3x4 object-to-world transform.
    pub transform: [f32; 12],
    /// Value `optixGetInstanceId` returns.
    pub id: u32,
    /// Offset of this instance's hit groups in the SBT.
    pub sbt_offset: u32,
    /// Visibility mask (8 bits); a ray hits the instance when `ray_mask & mask != 0`.
    pub mask: u8,
    /// `OptixInstanceFlags`.
    pub flags: u32,
}

impl Instance {
    /// The identity transform.
    pub const IDENTITY: [f32; 12] = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];

    /// `accel` with the identity transform, id 0, SBT offset 0, visible to every ray.
    pub fn new(accel: &Arc<Accel>) -> Self {
        Self { accel: accel.clone(), transform: Self::IDENTITY, id: 0, sbt_offset: 0, mask: 0xFF, flags: 0 }
    }
}

/// A built acceleration structure.
pub struct Accel {
    handle: sys::OptixTraversableHandle,
    buffer: DeviceBuffer<u8>,
    _children: Vec<Arc<Accel>>,
    ctx: Arc<DeviceContext>,
}

impl Accel {
    /// A geometry structure over `meshes`, built on `stream`.
    pub fn triangles(ctx: &Arc<DeviceContext>, stream: &Arc<Stream>, meshes: &[TriangleMesh<'_>], opts: BuildOptions) -> Result<Arc<Self>> {
        if meshes.is_empty() {
            return Err(Error::Invalid("a geometry structure needs at least one mesh"));
        }
        // OptiX reads these through pointers during the sizing and build calls below.
        let vertex_ptrs: Vec<sys::CUdeviceptr> = meshes.iter().map(|m| m.vertices.cu_deviceptr()).collect();
        let flags: Vec<u32> = meshes.iter().map(|m| m.flags.0).collect();
        let mut inputs = Vec::with_capacity(meshes.len());
        for (i, m) in meshes.iter().enumerate() {
            let mut input = sys::OptixBuildInput { type_: sys::OptixBuildInputType_OPTIX_BUILD_INPUT_TYPE_TRIANGLES, ..Default::default() };
            let (index_ptr, index_count, index_format) = match m.indices {
                Some(ix) => (ix.cu_deviceptr(), ix.len(), sys::OptixIndicesFormat_OPTIX_INDICES_FORMAT_UNSIGNED_INT3),
                None => (0, 0, sys::OptixIndicesFormat_OPTIX_INDICES_FORMAT_NONE),
            };
            input.__bindgen_anon_1.triangleArray = sys::OptixBuildInputTriangleArray {
                vertexBuffers: &vertex_ptrs[i],
                numVertices: u32::try_from(m.vertices.len()).map_err(|_| Error::Invalid("too many vertices"))?,
                vertexFormat: sys::OptixVertexFormat_OPTIX_VERTEX_FORMAT_FLOAT3,
                vertexStrideInBytes: 12,
                indexBuffer: index_ptr,
                numIndexTriplets: u32::try_from(index_count).map_err(|_| Error::Invalid("too many triangles"))?,
                indexFormat: index_format,
                indexStrideInBytes: if m.indices.is_some() { 12 } else { 0 },
                flags: &flags[i],
                numSbtRecords: 1,
                ..Default::default()
            };
            inputs.push(input);
        }
        Self::build(ctx, stream, &inputs, opts, Vec::new())
    }

    /// An instance structure over `instances`, built on `stream`.
    pub fn instances(ctx: &Arc<DeviceContext>, stream: &Arc<Stream>, instances: &[Instance], opts: BuildOptions) -> Result<Arc<Self>> {
        if instances.iter().any(|i| !Arc::ptr_eq(&i.accel.ctx, ctx)) {
            return Err(Error::Invalid("instance of a structure from another device context"));
        }
        let raw: Vec<sys::OptixInstance> = instances
            .iter()
            .map(|i| sys::OptixInstance {
                transform: i.transform,
                instanceId: i.id,
                sbtOffset: i.sbt_offset,
                visibilityMask: u32::from(i.mask),
                flags: i.flags,
                traversableHandle: i.accel.handle,
                pad: [0; 2],
            })
            .collect();
        // SAFETY: `OptixInstance` is repr(C), 80 bytes of integers and floats with no padding
        // (12 f32 + 4 u32 + u64 + 2 u32), so its bytes are all initialized.
        let bytes = unsafe { std::slice::from_raw_parts(raw.as_ptr().cast::<u8>(), std::mem::size_of_val(raw.as_slice())) };
        let device = DeviceBuffer::<u8>::from_host(stream, bytes)?;
        let mut input = sys::OptixBuildInput { type_: sys::OptixBuildInputType_OPTIX_BUILD_INPUT_TYPE_INSTANCES, ..Default::default() };
        input.__bindgen_anon_1.instanceArray = sys::OptixBuildInputInstanceArray {
            instances: device.cu_deviceptr(),
            numInstances: u32::try_from(raw.len()).map_err(|_| Error::Invalid("too many instances"))?,
            ..Default::default()
        };
        let children = instances.iter().map(|i| i.accel.clone()).collect();
        let accel = Self::build(ctx, stream, &[input], opts, children)?;
        // `from_host` memory is freed with the synchronous `cuMemFree`, which does not wait for
        // the stream; the build must finish reading the instance array first. (A compacting
        // build has already synchronized.)
        if !opts.compact {
            stream.synchronize()?;
        }
        drop(device);
        Ok(accel)
    }

    fn build(ctx: &Arc<DeviceContext>, stream: &Arc<Stream>, inputs: &[sys::OptixBuildInput], opts: BuildOptions, children: Vec<Arc<Accel>>) -> Result<Arc<Self>> {
        let t = ctx.table();
        let mut flags = if opts.prefer_fast_trace {
            sys::OptixBuildFlags_OPTIX_BUILD_FLAG_PREFER_FAST_TRACE
        } else {
            sys::OptixBuildFlags_OPTIX_BUILD_FLAG_PREFER_FAST_BUILD
        } as u32;
        if opts.compact {
            flags |= sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_COMPACTION as u32;
        }
        let accel_opts = sys::OptixAccelBuildOptions { buildFlags: flags, operation: sys::OptixBuildOperation_OPTIX_BUILD_OPERATION_BUILD, ..Default::default() };
        let n = u32::try_from(inputs.len()).map_err(|_| Error::Invalid("too many build inputs"))?;
        let mut sizes = sys::OptixAccelBufferSizes::default();
        ctx.cuda().bind_to_thread()?;
        // SAFETY: `inputs` and the buffers they point at are alive for the call.
        check("optixAccelComputeMemoryUsage", unsafe { entry!(t, optixAccelComputeMemoryUsage)(ctx.raw(), &accel_opts, inputs.as_ptr(), n, &mut sizes) })?;

        let peak = sizes.tempSizeInBytes.saturating_add(sizes.outputSizeInBytes);
        if let Some(available) = opts.budget
            && peak > available
        {
            return Err(Error::Budget { requested: peak, available });
        }
        // SAFETY: OptiX writes both buffers before anything reads them.
        let temp = unsafe { DeviceBuffer::<u8>::uninitialized_async(stream, sizes.tempSizeInBytes.max(1)) }?;
        // SAFETY: as above.
        let output = unsafe { DeviceBuffer::<u8>::uninitialized_async(stream, sizes.outputSizeInBytes) }?;
        let compacted_size = if opts.compact { Some(DeviceBuffer::<u64>::zeroed(stream, 1)?) } else { None };
        let emit = compacted_size.as_ref().map(|b| sys::OptixAccelEmitDesc {
            result: b.cu_deviceptr(),
            type_: sys::OptixAccelPropertyType_OPTIX_PROPERTY_TYPE_COMPACTED_SIZE,
        });
        if output.cu_deviceptr() % u64::from(sys::OPTIX_ACCEL_BUFFER_BYTE_ALIGNMENT) != 0 {
            return Err(Error::Invalid("allocator returned an output buffer below OptiX's 128-byte alignment"));
        }
        let mut handle = 0;
        // SAFETY: buffers are sized by the call above and live on `stream` until the build is
        // done (their frees are ordered after it); `inputs` live for the call.
        check("optixAccelBuild", unsafe {
            entry!(t, optixAccelBuild)(
                ctx.raw(),
                stream.cu_stream(),
                &accel_opts,
                inputs.as_ptr(),
                n,
                temp.cu_deviceptr(),
                sizes.tempSizeInBytes,
                output.cu_deviceptr(),
                sizes.outputSizeInBytes,
                &mut handle,
                emit.as_ref().map_or(std::ptr::null(), |e| e as *const _),
                u32::from(emit.is_some()),
            )
        })?;
        drop(temp);

        let Some(compacted_size) = compacted_size else {
            return Ok(Arc::new(Self { handle, buffer: output, _children: children, ctx: ctx.clone() }));
        };
        let size = usize::try_from(compacted_size.to_host_vec(stream)?[0]).map_err(|_| Error::Invalid("compacted size overflows usize"))?;
        if size == 0 || size >= sizes.outputSizeInBytes {
            return Ok(Arc::new(Self { handle, buffer: output, _children: children, ctx: ctx.clone() }));
        }
        // SAFETY: written by the compaction below before any read.
        let compact = unsafe { DeviceBuffer::<u8>::uninitialized_async(stream, size) }?;
        let mut compact_handle = 0;
        // SAFETY: `handle` was built above with ALLOW_COMPACTION; `compact` is `size` bytes on
        // `stream`; the source buffer's free is ordered after this call.
        check("optixAccelCompact", unsafe {
            entry!(t, optixAccelCompact)(ctx.raw(), stream.cu_stream(), handle, compact.cu_deviceptr(), size, &mut compact_handle)
        })?;
        drop(output);
        Ok(Arc::new(Self { handle: compact_handle, buffer: compact, _children: children, ctx: ctx.clone() }))
    }

    /// The handle device code traces against (`optixTrace`'s first argument).
    pub fn handle(&self) -> sys::OptixTraversableHandle {
        self.handle
    }

    /// Bytes the structure occupies on the device.
    pub fn size_bytes(&self) -> usize {
        self.buffer.len()
    }
}
