//! Acceleration structures: [`Accel`] over triangle meshes (GAS) or over [`Instance`]s (IAS).
//!
//! **What.** A build sizes the structure (`optixAccelComputeMemoryUsage`), checks the caller's
//! byte budget, takes temp memory from a caller-owned [`Scratch`] (grown on demand, reused across
//! builds), allocates the output on the caller's stream, builds and, with
//! [`BuildOptions::compact`], reads the compacted size back and compacts into a tighter buffer.
//! A structure built with [`BuildOptions::allow_update`] can be refitted in place after its
//! vertices or instance transforms move ([`Accel::refit_triangles`], [`Accel::refit_instances`]);
//! the topology (mesh, vertex, triangle and instance counts) must stay the same.
//! Input buffers are only read during a build; the result references nothing but its own buffer,
//! except an instance structure, which holds `Arc`s of the structures it instances.
//! **Ordering.** Output, compaction and instance buffers are allocated on the stream and freed
//! with `cuMemFreeAsync` on it, so frees are ordered after the work that uses them. A [`Scratch`]
//! belongs to one stream, which orders its reuse. Compaction synchronizes the stream once.

use crate::{DeviceContext, Error, Result, check, entry, sys};
use cuda_core::{DeviceBuffer, IntoResult, Stream};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

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
    /// Allow [`Accel::refit_triangles`] / [`Accel::refit_instances`] later (costs some trace
    /// speed and memory).
    pub allow_update: bool,
    /// New device bytes the build may allocate at peak (scratch growth + output); `None` admits
    /// any size. Checked after OptiX reports the sizes and before anything is allocated.
    pub budget: Option<usize>,
}

impl Default for BuildOptions {
    /// Fast trace, compacted, static, no budget: the right choice for static geometry.
    fn default() -> Self {
        Self { prefer_fast_trace: true, compact: true, allow_update: false, budget: None }
    }
}

impl BuildOptions {
    fn flags(&self) -> u32 {
        let mut flags = if self.prefer_fast_trace {
            sys::OptixBuildFlags_OPTIX_BUILD_FLAG_PREFER_FAST_TRACE
        } else {
            sys::OptixBuildFlags_OPTIX_BUILD_FLAG_PREFER_FAST_BUILD
        } as u32;
        if self.compact {
            flags |= sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_COMPACTION as u32;
        }
        if self.allow_update {
            flags |= sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_UPDATE as u32;
        }
        flags
    }
}

/// Temp memory for builds and refits, owned by the caller and reused across them on one stream.
/// It grows to the largest request and never shrinks.
pub struct Scratch {
    stream: Arc<Stream>,
    buffer: Option<DeviceBuffer<u8>>,
}

impl Scratch {
    /// Empty scratch for builds on `stream`.
    pub fn new(stream: &Arc<Stream>) -> Self {
        Self { stream: stream.clone(), buffer: None }
    }

    /// Bytes currently held.
    pub fn capacity(&self) -> usize {
        self.buffer.as_ref().map_or(0, |b| b.len())
    }

    /// Bytes a request for `bytes` would newly allocate.
    fn growth(&self, bytes: usize) -> usize {
        bytes.saturating_sub(self.capacity())
    }

    /// Device memory of at least `bytes` (at least one byte, so the pointer is never null).
    fn reserve(&mut self, stream: &Arc<Stream>, bytes: usize) -> Result<(sys::CUdeviceptr, usize)> {
        if !Arc::ptr_eq(stream, &self.stream) {
            return Err(Error::Invalid("scratch used on a different stream than it was created for"));
        }
        let bytes = bytes.max(1);
        if self.capacity() < bytes {
            // The old buffer's free is ordered after the work already queued on this stream.
            self.buffer = None;
            // SAFETY: OptiX writes temp memory before reading it.
            self.buffer = Some(unsafe { DeviceBuffer::<u8>::uninitialized_async(stream, bytes) }?);
        }
        let b = self.buffer.as_ref().ok_or(Error::Invalid("scratch buffer missing after reserve"))?;
        Ok((b.cu_deviceptr(), b.len()))
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

/// What a structure was built from; a refit must present the same shape.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Shape {
    /// Per mesh: vertex count, triangle count (0 = not indexed).
    Triangles(Vec<(u32, u32)>),
    /// Instance count.
    Instances(u32),
}

/// Build inputs plus the host arrays their pointers refer to (heap storage, so moves are fine).
struct Inputs {
    inputs: Vec<sys::OptixBuildInput>,
    shape: Shape,
    _vertex_ptrs: Vec<sys::CUdeviceptr>,
    _flags: Vec<u32>,
    _instances: Option<DeviceBuffer<u8>>,
}

fn triangle_inputs(meshes: &[TriangleMesh<'_>]) -> Result<Inputs> {
    if meshes.is_empty() {
        return Err(Error::Invalid("a geometry structure needs at least one mesh"));
    }
    let vertex_ptrs: Vec<sys::CUdeviceptr> = meshes.iter().map(|m| m.vertices.cu_deviceptr()).collect();
    let flags: Vec<u32> = meshes.iter().map(|m| m.flags.0).collect();
    let mut inputs = Vec::with_capacity(meshes.len());
    let mut shape = Vec::with_capacity(meshes.len());
    for (i, m) in meshes.iter().enumerate() {
        let vertices = u32::try_from(m.vertices.len()).map_err(|_| Error::Invalid("too many vertices"))?;
        let (index_ptr, triangles, index_format) = match m.indices {
            Some(ix) => (
                ix.cu_deviceptr(),
                u32::try_from(ix.len()).map_err(|_| Error::Invalid("too many triangles"))?,
                sys::OptixIndicesFormat_OPTIX_INDICES_FORMAT_UNSIGNED_INT3,
            ),
            None => (0, 0, sys::OptixIndicesFormat_OPTIX_INDICES_FORMAT_NONE),
        };
        let mut input = sys::OptixBuildInput { type_: sys::OptixBuildInputType_OPTIX_BUILD_INPUT_TYPE_TRIANGLES, ..Default::default() };
        input.__bindgen_anon_1.triangleArray = sys::OptixBuildInputTriangleArray {
            vertexBuffers: &vertex_ptrs[i],
            numVertices: vertices,
            vertexFormat: sys::OptixVertexFormat_OPTIX_VERTEX_FORMAT_FLOAT3,
            vertexStrideInBytes: 12,
            indexBuffer: index_ptr,
            numIndexTriplets: triangles,
            indexFormat: index_format,
            indexStrideInBytes: if m.indices.is_some() { 12 } else { 0 },
            flags: &flags[i],
            numSbtRecords: 1,
            ..Default::default()
        };
        inputs.push(input);
        shape.push((vertices, triangles));
    }
    Ok(Inputs { inputs, shape: Shape::Triangles(shape), _vertex_ptrs: vertex_ptrs, _flags: flags, _instances: None })
}

fn instance_inputs(ctx: &Arc<DeviceContext>, stream: &Arc<Stream>, instances: &[Instance]) -> Result<Inputs> {
    if instances.is_empty() {
        return Err(Error::Invalid("an instance structure needs at least one instance"));
    }
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
            traversableHandle: i.accel.handle(),
            pad: [0; 2],
        })
        .collect();
    let count = u32::try_from(raw.len()).map_err(|_| Error::Invalid("too many instances"))?;
    let bytes = size_of_val(raw.as_slice());
    // SAFETY: filled by the copy below before the build reads it.
    let device = unsafe { DeviceBuffer::<u8>::uninitialized_async(stream, bytes) }?;
    // SAFETY: `raw` is `bytes` initialized bytes (repr(C) floats/integers, no padding: 12 f32 +
    // 4 u32 + u64 + 2 u32 = 80 bytes); a pageable source is staged before the call returns, so
    // `raw` may be dropped afterwards; the destination holds `bytes` bytes.
    unsafe { cuda_core::sys::cuMemcpyHtoDAsync_v2(device.cu_deviceptr(), raw.as_ptr().cast(), bytes, stream.cu_stream()) }.result()?;
    let mut input = sys::OptixBuildInput { type_: sys::OptixBuildInputType_OPTIX_BUILD_INPUT_TYPE_INSTANCES, ..Default::default() };
    input.__bindgen_anon_1.instanceArray = sys::OptixBuildInputInstanceArray { instances: device.cu_deviceptr(), numInstances: count, ..Default::default() };
    Ok(Inputs { inputs: vec![input], shape: Shape::Instances(count), _vertex_ptrs: Vec::new(), _flags: Vec::new(), _instances: Some(device) })
}

const _: () = assert!(size_of::<sys::OptixInstance>() == 80);

/// A built acceleration structure.
pub struct Accel {
    handle: AtomicU64,
    buffer: DeviceBuffer<u8>,
    flags: u32,
    shape: Shape,
    children: Mutex<Vec<Arc<Accel>>>,
    ctx: Arc<DeviceContext>,
}

impl Accel {
    /// A geometry structure over `meshes`, built on `stream` with temp memory from `scratch`.
    pub fn triangles(ctx: &Arc<DeviceContext>, stream: &Arc<Stream>, scratch: &mut Scratch, meshes: &[TriangleMesh<'_>], opts: BuildOptions) -> Result<Arc<Self>> {
        let inputs = triangle_inputs(meshes)?;
        Self::build(ctx, stream, scratch, inputs, opts, Vec::new())
    }

    /// An instance structure over `instances`, built on `stream` with temp memory from `scratch`.
    pub fn instances(ctx: &Arc<DeviceContext>, stream: &Arc<Stream>, scratch: &mut Scratch, instances: &[Instance], opts: BuildOptions) -> Result<Arc<Self>> {
        let inputs = instance_inputs(ctx, stream, instances)?;
        let children = instances.iter().map(|i| i.accel.clone()).collect();
        Self::build(ctx, stream, scratch, inputs, opts, children)
    }

    fn sizes(ctx: &DeviceContext, accel_opts: &sys::OptixAccelBuildOptions, inputs: &[sys::OptixBuildInput]) -> Result<sys::OptixAccelBufferSizes> {
        let n = u32::try_from(inputs.len()).map_err(|_| Error::Invalid("too many build inputs"))?;
        let mut sizes = sys::OptixAccelBufferSizes::default();
        ctx.cuda().bind_to_thread()?;
        // SAFETY: `inputs` and the buffers they point at are alive for the call.
        check("optixAccelComputeMemoryUsage", unsafe {
            entry!(ctx.table(), optixAccelComputeMemoryUsage)(ctx.raw(), accel_opts, inputs.as_ptr(), n, &mut sizes)
        })?;
        Ok(sizes)
    }

    fn build(ctx: &Arc<DeviceContext>, stream: &Arc<Stream>, scratch: &mut Scratch, inputs: Inputs, opts: BuildOptions, children: Vec<Arc<Accel>>) -> Result<Arc<Self>> {
        let t = ctx.table();
        let flags = opts.flags();
        let accel_opts = sys::OptixAccelBuildOptions { buildFlags: flags, operation: sys::OptixBuildOperation_OPTIX_BUILD_OPERATION_BUILD, ..Default::default() };
        let sizes = Self::sizes(ctx, &accel_opts, &inputs.inputs)?;
        let peak = scratch.growth(sizes.tempSizeInBytes).saturating_add(sizes.outputSizeInBytes);
        if let Some(available) = opts.budget
            && peak > available
        {
            return Err(Error::Budget { requested: peak, available });
        }
        let (temp, temp_len) = scratch.reserve(stream, sizes.tempSizeInBytes)?;
        // SAFETY: OptiX writes the output before anything reads it.
        let output = unsafe { DeviceBuffer::<u8>::uninitialized_async(stream, sizes.outputSizeInBytes) }?;
        if output.cu_deviceptr() % u64::from(sys::OPTIX_ACCEL_BUFFER_BYTE_ALIGNMENT) != 0 {
            return Err(Error::Invalid("allocator returned an output buffer below OptiX's 128-byte alignment"));
        }
        let compacted_size = if opts.compact { Some(DeviceBuffer::<u64>::zeroed(stream, 1)?) } else { None };
        let emit = compacted_size.as_ref().map(|b| sys::OptixAccelEmitDesc {
            result: b.cu_deviceptr(),
            type_: sys::OptixAccelPropertyType_OPTIX_PROPERTY_TYPE_COMPACTED_SIZE,
        });
        let n = inputs.inputs.len() as u32; // bounded by `sizes` above
        let mut handle = 0;
        // SAFETY: buffers are sized by OptiX above and ordered on `stream` (their frees come after
        // the build); `inputs` and what they point at live for the call.
        check("optixAccelBuild", unsafe {
            entry!(t, optixAccelBuild)(
                ctx.raw(),
                stream.cu_stream(),
                &accel_opts,
                inputs.inputs.as_ptr(),
                n,
                temp,
                temp_len,
                output.cu_deviceptr(),
                sizes.outputSizeInBytes,
                &mut handle,
                emit.as_ref().map_or(std::ptr::null(), |e| e as *const _),
                u32::from(emit.is_some()),
            )
        })?;
        let make = |handle, buffer| Arc::new(Self { handle: AtomicU64::new(handle), buffer, flags, shape: inputs.shape.clone(), children: Mutex::new(children.clone()), ctx: ctx.clone() });

        let Some(compacted_size) = compacted_size else {
            return Ok(make(handle, output));
        };
        let size = usize::try_from(compacted_size.to_host_vec(stream)?[0]).map_err(|_| Error::Invalid("compacted size overflows usize"))?;
        if size == 0 || size >= sizes.outputSizeInBytes {
            return Ok(make(handle, output));
        }
        // SAFETY: written by the compaction below before any read.
        let compact = unsafe { DeviceBuffer::<u8>::uninitialized_async(stream, size) }?;
        let mut compact_handle = 0;
        // SAFETY: `handle` was built with ALLOW_COMPACTION; `compact` holds `size` bytes on
        // `stream`; the source buffer's free is ordered after this call.
        check("optixAccelCompact", unsafe {
            entry!(t, optixAccelCompact)(ctx.raw(), stream.cu_stream(), handle, compact.cu_deviceptr(), size, &mut compact_handle)
        })?;
        drop(output);
        Ok(make(compact_handle, compact))
    }

    /// Refits a geometry structure to moved vertices: same meshes, vertex and triangle counts.
    ///
    /// # Safety
    ///
    /// No launch may read this structure (directly or through an instance structure) while the
    /// refit runs: order such launches on `stream` or synchronize. Instance structures over this
    /// one keep stale bounds until they are refitted or rebuilt too.
    pub unsafe fn refit_triangles(&self, stream: &Arc<Stream>, scratch: &mut Scratch, meshes: &[TriangleMesh<'_>]) -> Result<()> {
        let inputs = triangle_inputs(meshes)?;
        // SAFETY: forwarded contract.
        unsafe { self.update(stream, scratch, inputs) }
    }

    /// Refits an instance structure to new transforms, masks, SBT offsets or instanced
    /// structures; the instance count must stay the same.
    ///
    /// # Safety
    ///
    /// As [`Accel::refit_triangles`].
    pub unsafe fn refit_instances(&self, stream: &Arc<Stream>, scratch: &mut Scratch, instances: &[Instance]) -> Result<()> {
        let inputs = instance_inputs(&self.ctx, stream, instances)?;
        // SAFETY: forwarded contract.
        unsafe { self.update(stream, scratch, inputs)? };
        let mut children = self.children.lock().map_err(|_| Error::Invalid("instance list poisoned by a panic"))?;
        *children = instances.iter().map(|i| i.accel.clone()).collect();
        Ok(())
    }

    /// # Safety
    ///
    /// See [`Accel::refit_triangles`].
    unsafe fn update(&self, stream: &Arc<Stream>, scratch: &mut Scratch, inputs: Inputs) -> Result<()> {
        if self.flags & sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_UPDATE as u32 == 0 {
            return Err(Error::Invalid("structure was not built with BuildOptions::allow_update"));
        }
        if inputs.shape != self.shape {
            return Err(Error::Invalid("refit inputs differ in shape from the build (counts or kind)"));
        }
        let accel_opts = sys::OptixAccelBuildOptions { buildFlags: self.flags, operation: sys::OptixBuildOperation_OPTIX_BUILD_OPERATION_UPDATE, ..Default::default() };
        let sizes = Self::sizes(&self.ctx, &accel_opts, &inputs.inputs)?;
        let (temp, temp_len) = scratch.reserve(stream, sizes.tempUpdateSizeInBytes)?;
        let mut handle = self.handle();
        let n = inputs.inputs.len() as u32; // bounded by `sizes` above
        // SAFETY: updates in place into the structure's own buffer (its full, possibly compacted,
        // size); the caller keeps readers off it; inputs live for the call.
        check("optixAccelBuild(update)", unsafe {
            entry!(self.ctx.table(), optixAccelBuild)(
                self.ctx.raw(),
                stream.cu_stream(),
                &accel_opts,
                inputs.inputs.as_ptr(),
                n,
                temp,
                temp_len,
                self.buffer.cu_deviceptr(),
                self.buffer.len(),
                &mut handle,
                std::ptr::null(),
                0,
            )
        })?;
        self.handle.store(handle, Ordering::Release);
        Ok(())
    }

    /// The handle device code traces against (`optixTrace`'s first argument).
    pub fn handle(&self) -> sys::OptixTraversableHandle {
        self.handle.load(Ordering::Acquire)
    }

    /// Bytes the structure occupies on the device.
    pub fn size_bytes(&self) -> usize {
        self.buffer.len()
    }
}
