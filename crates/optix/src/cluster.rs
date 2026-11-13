//! Cluster acceleration structures (RTX Mega Geometry): [`ClusterAccel`] builds CLAS (cluster
//! acceleration structures), cluster templates, or GASes over clusters with
//! `optixClusterAccelBuild`, in every build type and mode the OptiX 9.1 header defines.
//!
//! **What.** A cluster build is indirect: the host gives the build kind and its limits
//! ([`ClusterInput`]: [`TriangleLimits`], [`GridLimits`] or [`ClusterLimits`]), the per-object
//! arguments live on the device ([`ClusterArgs`] of the matching `OptixClusterAccelBuildInput*Args`
//! type, uploaded from the host or written by a kernel, with an optional device-side count).
//! Modes: implicit destinations ([`ClusterAccel::build`]: one output buffer sized by
//! `optixClusterAccelComputeMemoryUsage`), get-sizes ([`ClusterAccel::get_sizes`]: exact
//! per-object sizes) and explicit destinations ([`ClusterAccel::build_explicit`]: objects packed
//! at aligned offsets of one tight buffer from those sizes). Every result owns its output, its
//! per-object handle array (CLAS / template addresses, or GAS traversable handles) and, on
//! request, per-object sizes; it keeps `Arc`s of the clusters and micromap arrays its arguments
//! reference.
//! **Pipeline.** triangles -> CLAS ([`ClusterKind::ClustersFromTriangles`]) or templates
//! ([`ClusterKind::TemplatesFromTriangles`] / [`ClusterKind::TemplatesFromGrids`]) -> CLAS from
//! templates ([`ClusterKind::ClustersFromTemplates`]) -> GAS over CLAS handles
//! ([`ClusterKind::GasesFromClusters`]); a stage's [`ClusterAccel::handles`] feed the next
//! stage's arguments.
//! **Budgets and alignment.** Peak new device bytes (scratch growth, output, handle/size/address
//! arrays) are checked against the caller's budget before anything is allocated; outputs are
//! aligned to `OPTIX_ACCEL_BUFFER_BYTE_ALIGNMENT`, which covers every cluster output type, and
//! temp memory comes from a [`Scratch`] of the build stream.
//! **Used by** render-rs (cluster geometry) and the nv-rs OptiX tests.

use crate::accel::{Scratch, VertexFormat, alloc, count, same_ctx, upload, within};
use crate::micromap::OpacityMicromapArray;
use crate::{DeviceContext, Error, Result, check, entry, sys};
use cuda_core::{DeviceBuffer, DeviceCopy, Stream};
use std::marker::PhantomData;
use std::sync::Arc;

/// Host-side cluster build flags (`OptixClusterAccelBuildFlags`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClusterFlags(pub u32);

impl ClusterFlags {
    /// No preference.
    pub const NONE: Self = Self(sys::OptixClusterAccelBuildFlags_OPTIX_CLUSTER_ACCEL_BUILD_FLAG_NONE as u32);
    /// Favour trace speed.
    pub const PREFER_FAST_TRACE: Self = Self(sys::OptixClusterAccelBuildFlags_OPTIX_CLUSTER_ACCEL_BUILD_FLAG_PREFER_FAST_TRACE as u32);
    /// Favour build speed.
    pub const PREFER_FAST_BUILD: Self = Self(sys::OptixClusterAccelBuildFlags_OPTIX_CLUSTER_ACCEL_BUILD_FLAG_PREFER_FAST_BUILD as u32);
    /// Arguments may reference opacity micromaps.
    pub const ALLOW_OPACITY_MICROMAPS: Self = Self(sys::OptixClusterAccelBuildFlags_OPTIX_CLUSTER_ACCEL_BUILD_FLAG_ALLOW_OPACITY_MICROMAPS as u32);

    fn raw(self) -> sys::OptixClusterAccelBuildFlags {
        self.0 as sys::OptixClusterAccelBuildFlags
    }
}

/// What a cluster build produces from what (`OptixClusterAccelBuildType`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClusterKind {
    /// GASes over CLAS handles (args: `OptixClusterAccelBuildInputClustersArgs`).
    GasesFromClusters,
    /// CLAS from triangles (args: `OptixClusterAccelBuildInputTrianglesArgs`).
    ClustersFromTriangles,
    /// Templates from triangles (args: `OptixClusterAccelBuildInputTrianglesArgs`).
    TemplatesFromTriangles,
    /// CLAS instantiated from templates (args: `OptixClusterAccelBuildInputTemplatesArgs`).
    ClustersFromTemplates,
    /// Templates from structured grids (args: `OptixClusterAccelBuildInputGridsArgs`).
    TemplatesFromGrids,
}

impl ClusterKind {
    fn raw(self) -> sys::OptixClusterAccelBuildType {
        match self {
            Self::GasesFromClusters => sys::OptixClusterAccelBuildType_OPTIX_CLUSTER_ACCEL_BUILD_TYPE_GASES_FROM_CLUSTERS,
            Self::ClustersFromTriangles => sys::OptixClusterAccelBuildType_OPTIX_CLUSTER_ACCEL_BUILD_TYPE_CLUSTERS_FROM_TRIANGLES,
            Self::TemplatesFromTriangles => sys::OptixClusterAccelBuildType_OPTIX_CLUSTER_ACCEL_BUILD_TYPE_TEMPLATES_FROM_TRIANGLES,
            Self::ClustersFromTemplates => sys::OptixClusterAccelBuildType_OPTIX_CLUSTER_ACCEL_BUILD_TYPE_CLUSTERS_FROM_TEMPLATES,
            Self::TemplatesFromGrids => sys::OptixClusterAccelBuildType_OPTIX_CLUSTER_ACCEL_BUILD_TYPE_TEMPLATES_FROM_GRIDS,
        }
    }
}

/// Which output target a size query is for (`OptixClusterAccelBuildMode`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClusterMode {
    /// One output buffer holding every object.
    Implicit,
    /// Caller-placed objects; the output size is per object.
    Explicit,
    /// Exact per-object sizes for a later explicit build (temp only).
    GetSizes,
}

impl ClusterMode {
    fn raw(self) -> sys::OptixClusterAccelBuildMode {
        match self {
            Self::Implicit => sys::OptixClusterAccelBuildMode_OPTIX_CLUSTER_ACCEL_BUILD_MODE_IMPLICIT_DESTINATIONS,
            Self::Explicit => sys::OptixClusterAccelBuildMode_OPTIX_CLUSTER_ACCEL_BUILD_MODE_EXPLICIT_DESTINATIONS,
            Self::GetSizes => sys::OptixClusterAccelBuildMode_OPTIX_CLUSTER_ACCEL_BUILD_MODE_GET_SIZES,
        }
    }
}

/// Limits of triangle-based builds (`OptixClusterAccelBuildInputTriangles`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TriangleLimits {
    /// Build flags.
    pub flags: ClusterFlags,
    /// Maximum number of arguments (objects) per build.
    pub max_args: u32,
    /// Vertex format of every argument.
    pub vertex_format: VertexFormat,
    /// Largest SBT index used, including base, per-primitive and template offsets.
    pub max_sbt_index: u32,
    /// Largest number of distinct SBT indices in one cluster.
    pub max_unique_sbt_per_arg: u32,
    /// Largest triangle count of one argument.
    pub max_triangles_per_arg: u32,
    /// Largest vertex count of one argument.
    pub max_vertices_per_arg: u32,
    /// Bound on triangles over all arguments (0 = `max_triangles_per_arg * max_args`).
    pub max_total_triangles: u32,
    /// Bound on vertices over all arguments (0 = `max_vertices_per_arg * max_args`).
    pub max_total_vertices: u32,
    /// Lower bound of the position mantissa bits arguments truncate.
    pub min_truncate_bits: u32,
}

impl Default for TriangleLimits {
    /// `Float3` vertices, one SBT index, all bounds zero (to be filled).
    fn default() -> Self {
        Self {
            flags: ClusterFlags::NONE,
            max_args: 0,
            vertex_format: VertexFormat::Float3,
            max_sbt_index: 0,
            max_unique_sbt_per_arg: 1,
            max_triangles_per_arg: 0,
            max_vertices_per_arg: 0,
            max_total_triangles: 0,
            max_total_vertices: 0,
            min_truncate_bits: 0,
        }
    }
}

/// Limits of grid template builds (`OptixClusterAccelBuildInputGrids`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridLimits {
    /// Build flags.
    pub flags: ClusterFlags,
    /// Maximum number of arguments (grids) per build.
    pub max_args: u32,
    /// Vertex format used at instantiation.
    pub vertex_format: VertexFormat,
    /// Largest SBT index used, including base, per-primitive and template offsets.
    pub max_sbt_index: u32,
    /// Largest number of edge segments along a grid's width.
    pub max_width: u32,
    /// Largest number of edge segments along a grid's height.
    pub max_height: u32,
}

/// Limits of GAS-over-cluster builds (`OptixClusterAccelBuildInputClusters`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClusterLimits {
    /// Build flags.
    pub flags: ClusterFlags,
    /// Maximum number of arguments (GASes) per build.
    pub max_args: u32,
    /// Bound on CLAS handles over all arguments.
    pub max_total_clusters: u32,
    /// Bound on CLAS handles of one argument.
    pub max_clusters_per_arg: u32,
}

/// The kind of a cluster build with its limits (`OptixClusterAccelBuildInput`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClusterInput {
    /// CLAS from triangles.
    ClustersFromTriangles(TriangleLimits),
    /// Templates from triangles.
    TemplatesFromTriangles(TriangleLimits),
    /// CLAS from templates.
    ClustersFromTemplates(TriangleLimits),
    /// Templates from grids.
    TemplatesFromGrids(GridLimits),
    /// GASes over CLAS.
    GasesFromClusters(ClusterLimits),
}

impl ClusterInput {
    /// The build kind.
    pub fn kind(&self) -> ClusterKind {
        match self {
            Self::ClustersFromTriangles(_) => ClusterKind::ClustersFromTriangles,
            Self::TemplatesFromTriangles(_) => ClusterKind::TemplatesFromTriangles,
            Self::ClustersFromTemplates(_) => ClusterKind::ClustersFromTemplates,
            Self::TemplatesFromGrids(_) => ClusterKind::TemplatesFromGrids,
            Self::GasesFromClusters(_) => ClusterKind::GasesFromClusters,
        }
    }

    /// Maximum number of arguments one build reads.
    pub fn max_args(&self) -> u32 {
        match self {
            Self::ClustersFromTriangles(l) | Self::TemplatesFromTriangles(l) | Self::ClustersFromTemplates(l) => l.max_args,
            Self::TemplatesFromGrids(l) => l.max_args,
            Self::GasesFromClusters(l) => l.max_args,
        }
    }

    fn to_sys(self) -> sys::OptixClusterAccelBuildInput {
        let mut raw = sys::OptixClusterAccelBuildInput { type_: self.kind().raw(), ..Default::default() };
        match self {
            Self::ClustersFromTriangles(l) | Self::TemplatesFromTriangles(l) | Self::ClustersFromTemplates(l) => {
                raw.__bindgen_anon_1.triangles = sys::OptixClusterAccelBuildInputTriangles {
                    flags: l.flags.raw(),
                    maxArgCount: l.max_args,
                    vertexFormat: l.vertex_format.raw(),
                    maxSbtIndexValue: l.max_sbt_index,
                    maxUniqueSbtIndexCountPerArg: l.max_unique_sbt_per_arg,
                    maxTriangleCountPerArg: l.max_triangles_per_arg,
                    maxVertexCountPerArg: l.max_vertices_per_arg,
                    maxTotalTriangleCount: l.max_total_triangles,
                    maxTotalVertexCount: l.max_total_vertices,
                    minPositionTruncateBitCount: l.min_truncate_bits,
                };
            }
            Self::TemplatesFromGrids(l) => {
                raw.__bindgen_anon_1.grids = sys::OptixClusterAccelBuildInputGrids {
                    flags: l.flags.raw(),
                    maxArgCount: l.max_args,
                    vertexFormat: l.vertex_format.raw(),
                    maxSbtIndexValue: l.max_sbt_index,
                    maxWidth: l.max_width,
                    maxHeight: l.max_height,
                };
            }
            Self::GasesFromClusters(l) => {
                raw.__bindgen_anon_1.clusters = sys::OptixClusterAccelBuildInputClusters {
                    flags: l.flags.raw(),
                    maxArgCount: l.max_args,
                    maxTotalClusterCount: l.max_total_clusters,
                    maxClusterCountPerArg: l.max_clusters_per_arg,
                };
            }
        }
        raw
    }
}

mod sealed {
    pub trait Sealed {}
}

/// A device argument type of cluster builds: one of the `OptixClusterAccelBuildInput*Args`.
pub trait ClusterArg: sealed::Sealed + Copy {
    /// Whether builds of `kind` read arguments of this type.
    fn accepts(kind: ClusterKind) -> bool;
}

macro_rules! cluster_arg {
    ($ty:ty, $size:expr, $($kind:ident)|+) => {
        impl sealed::Sealed for $ty {}
        impl ClusterArg for $ty {
            fn accepts(kind: ClusterKind) -> bool {
                matches!(kind, $(ClusterKind::$kind)|+)
            }
        }
        // Field sizes sum to this (bindgen layout), so the type has no padding and uploads
        // byte-exact.
        const _: () = assert!(size_of::<$ty>() == $size);
    };
}

// u32 id, u32 flags, u32 bitfield, u32 primitive info, 4 x u16 strides, 6 x u64 pointers.
cluster_arg!(sys::OptixClusterAccelBuildInputTrianglesArgs, 72, ClustersFromTriangles | TemplatesFromTriangles);
// u32 id, u32 flags, u32 primitive info, u32 bitfield, 2 x u8 dimensions, u16 reserved.
cluster_arg!(sys::OptixClusterAccelBuildInputGridsArgs, 20, TemplatesFromGrids);
// 2 x u32 offsets, 2 x u64 pointers, 2 x u32 stride/reserved.
cluster_arg!(sys::OptixClusterAccelBuildInputTemplatesArgs, 32, ClustersFromTemplates);
// 2 x u32 count/stride, u64 pointer.
cluster_arg!(sys::OptixClusterAccelBuildInputClustersArgs, 16, GasesFromClusters);

/// Device-resident arguments of a cluster build, with an optional device-side argument count.
pub struct ClusterArgs<T: ClusterArg> {
    buffer: DeviceBuffer<u8>,
    len: u32,
    count: Option<DeviceBuffer<u32>>,
    /// The stream an upload was ordered on; `None` for device-written arguments.
    stream: Option<Arc<Stream>>,
    _t: PhantomData<T>,
}

impl<T: ClusterArg> ClusterArgs<T> {
    /// Uploads host-built arguments on `stream`; builds reading them must run on `stream`.
    pub fn upload(stream: &Arc<Stream>, args: &[T]) -> Result<Self> {
        let len = count(args.len(), "too many cluster arguments")?;
        // SAFETY: every `ClusterArg` type has no padding (asserted with its impl).
        let buffer = unsafe { upload(stream, args) }?;
        Ok(Self { buffer, len, count: None, stream: Some(stream.clone()), _t: PhantomData })
    }

    /// Wraps arguments a kernel writes: `buffer` holds `len` values of `T`; `count`, if given,
    /// holds one `u32` with the number of arguments to build.
    ///
    /// # Safety
    ///
    /// Before any build reads them (written earlier or ordered before the build's stream),
    /// `buffer` must hold `len` valid `T` values and `count` a value no greater than the build's
    /// `max_args` or `len`.
    pub unsafe fn from_device(buffer: DeviceBuffer<u8>, len: u32, count: Option<DeviceBuffer<u32>>) -> Result<Self> {
        if (len as usize).checked_mul(size_of::<T>()) != Some(buffer.len()) {
            return Err(Error::Invalid("cluster argument buffer size is not len * size_of::<T>()"));
        }
        if count.as_ref().is_some_and(|c| c.len() != 1) {
            return Err(Error::Invalid("cluster argument count buffer holds exactly one u32"));
        }
        Ok(Self { buffer, len, count, stream: None, _t: PhantomData })
    }

    /// The argument bytes (for kernels that rewrite them in place).
    pub fn buffer(&self) -> &DeviceBuffer<u8> {
        &self.buffer
    }

    /// Number of arguments the buffer holds.
    pub fn len(&self) -> u32 {
        self.len
    }

    /// Whether the buffer holds no arguments.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// Sizes OptiX reports for a cluster build mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClusterSizes {
    /// Output bytes: all objects (implicit) or one object (explicit); 0 for get-sizes.
    pub output: usize,
    /// Temp bytes.
    pub temp: usize,
}

/// Objects a build's arguments reference, kept alive by its result.
#[derive(Clone, Copy, Default)]
pub struct ClusterRefs<'a> {
    /// CLAS (for GASes over clusters) or templates (for CLAS from templates).
    pub clusters: &'a [Arc<ClusterAccel>],
    /// Micromap arrays referenced by triangle arguments.
    pub micromaps: &'a [Arc<OpacityMicromapArray>],
}

/// How a cluster build allocates and what it keeps.
#[derive(Clone, Copy, Default)]
pub struct ClusterOptions<'a> {
    /// What the arguments reference.
    pub refs: ClusterRefs<'a>,
    /// New device bytes the build may allocate at peak; `None` admits any size.
    pub budget: Option<usize>,
    /// Also write per-object output sizes ([`ClusterAccel::output_sizes`]).
    pub sizes: bool,
}

/// A zero-filled stream-ordered buffer of `len` (at least one) elements, without synchronizing.
fn zeroed_async<T: DeviceCopy>(stream: &Arc<Stream>, len: usize) -> Result<DeviceBuffer<T>> {
    // SAFETY: the memset below writes every byte before anything reads, and `DeviceCopy` types
    // have a valid all-zero representation.
    let buffer = unsafe { DeviceBuffer::<T>::uninitialized_async(stream, len.max(1)) }?;
    // SAFETY: `buffer` holds `num_bytes` bytes in the stream's context.
    unsafe { cuda_core::memory::memset_d8_async(buffer.cu_deviceptr(), 0, buffer.num_bytes(), stream.cu_stream()) }?;
    Ok(buffer)
}

/// Built cluster objects of one kind: CLAS, templates or GASes, owning their memory.
pub struct ClusterAccel {
    kind: ClusterKind,
    buffer: DeviceBuffer<u8>,
    handles: DeviceBuffer<u64>,
    sizes: Option<DeviceBuffer<u32>>,
    _clusters: Vec<Arc<ClusterAccel>>,
    _micromaps: Vec<Arc<OpacityMicromapArray>>,
    ctx: Arc<DeviceContext>,
}

impl ClusterAccel {
    /// Conservative sizes of a build of `input` in `mode` (`optixClusterAccelComputeMemoryUsage`).
    pub fn memory_usage(ctx: &DeviceContext, mode: ClusterMode, input: &ClusterInput) -> Result<ClusterSizes> {
        let raw = input.to_sys();
        let mut sizes = sys::OptixAccelBufferSizes::default();
        ctx.cuda().bind_to_thread()?;
        // SAFETY: `raw` lives for the call; `sizes` is writable.
        check("optixClusterAccelComputeMemoryUsage", unsafe {
            entry!(ctx.table(), optixClusterAccelComputeMemoryUsage)(ctx.raw(), mode.raw(), &raw, &mut sizes)
        })?;
        Ok(ClusterSizes { output: sizes.outputSizeInBytes, temp: sizes.tempSizeInBytes })
    }

    /// Validation shared by every mode.
    fn check_args<T: ClusterArg>(ctx: &Arc<DeviceContext>, stream: &Arc<Stream>, input: &ClusterInput, args: &ClusterArgs<T>, refs: &ClusterRefs<'_>) -> Result<usize> {
        same_ctx(stream, ctx)?;
        if !T::accepts(input.kind()) {
            return Err(Error::Invalid("cluster argument type does not match the build kind"));
        }
        let n = input.max_args();
        if n == 0 {
            return Err(Error::Invalid("a cluster build needs max_args > 0"));
        }
        // Without a device count OptiX reads `max_args` arguments; with one it reads up to that.
        if args.len < n {
            return Err(Error::Invalid("cluster argument buffer holds fewer than max_args arguments"));
        }
        if args.stream.as_ref().is_some_and(|s| !Arc::ptr_eq(s, stream)) {
            return Err(Error::Invalid("cluster arguments were uploaded on a different stream than the build"));
        }
        if refs.clusters.iter().any(|c| !Arc::ptr_eq(&c.ctx, ctx)) || refs.micromaps.iter().any(|m| !Arc::ptr_eq(m.context(), ctx)) {
            return Err(Error::Invalid("cluster references from another device context"));
        }
        Ok(n as usize)
    }

    /// # Safety
    ///
    /// `desc` names buffers sized for `input`, ordered on `stream`; the arguments are valid per
    /// the public builds' contract.
    unsafe fn run<T: ClusterArg>(ctx: &DeviceContext, stream: &Stream, desc: &sys::OptixClusterAccelBuildModeDesc, input: &ClusterInput, args: &ClusterArgs<T>) -> Result<()> {
        let raw = input.to_sys();
        ctx.cuda().bind_to_thread()?;
        // SAFETY: per the contract; `desc` and `raw` live for the call.
        check("optixClusterAccelBuild", unsafe {
            entry!(ctx.table(), optixClusterAccelBuild)(
                ctx.raw(),
                stream.cu_stream(),
                desc,
                &raw,
                args.buffer.cu_deviceptr(),
                args.count.as_ref().map_or(0, |c| c.cu_deviceptr()),
                size_of::<T>() as u32,
            )
        })
    }

    /// Builds up to `max_args` objects into one output buffer (implicit destinations).
    ///
    /// # Safety
    ///
    /// OptiX cannot check device arguments: until the build completes on `stream`, every device
    /// pointer in `args` (indices, vertices, primitive infos, bounding boxes, templates, CLAS
    /// handles, micromap arrays and indices) must be valid, written before (or ordered before)
    /// `stream`, and within the counts and limits of `input`; a device count must not exceed
    /// `max_args`. Every CLAS, template and micromap array the arguments reference must be in
    /// `opts.refs` so the result keeps it alive.
    pub unsafe fn build<T: ClusterArg>(ctx: &Arc<DeviceContext>, stream: &Arc<Stream>, scratch: &mut Scratch, input: &ClusterInput, args: &ClusterArgs<T>, opts: ClusterOptions<'_>) -> Result<Arc<Self>> {
        let n = Self::check_args(ctx, stream, input, args, &opts.refs)?;
        let sizes = Self::memory_usage(ctx, ClusterMode::Implicit, input)?;
        let arrays = n.saturating_mul(size_of::<u64>()).saturating_add(if opts.sizes { n.saturating_mul(size_of::<u32>()) } else { 0 });
        within(opts.budget, scratch.growth(sizes.temp).saturating_add(sizes.output).saturating_add(arrays))?;
        let (temp, temp_len) = scratch.reserve(stream, sizes.temp)?;
        // SAFETY: OptiX writes the objects before anything reads them; the accel alignment
        // covers CLAS and GAS (128 bytes) and templates (32 bytes).
        let buffer = unsafe { alloc(stream, sizes.output, sys::OPTIX_ACCEL_BUFFER_BYTE_ALIGNMENT) }?;
        let handles = zeroed_async::<u64>(stream, n)?;
        let out_sizes = if opts.sizes { Some(zeroed_async::<u32>(stream, n)?) } else { None };
        let mut desc = sys::OptixClusterAccelBuildModeDesc { mode: ClusterMode::Implicit.raw(), ..Default::default() };
        desc.__bindgen_anon_1.implicitDest = sys::OptixClusterAccelBuildModeDescImplicitDest {
            outputBuffer: buffer.cu_deviceptr(),
            outputBufferSizeInBytes: buffer.len(),
            tempBuffer: temp,
            tempBufferSizeInBytes: temp_len,
            outputHandlesBuffer: handles.cu_deviceptr(),
            outputHandlesStrideInBytes: size_of::<u64>() as u32,
            outputSizesBuffer: out_sizes.as_ref().map_or(0, |s| s.cu_deviceptr()),
            outputSizesStrideInBytes: if out_sizes.is_some() { size_of::<u32>() as u32 } else { 0 },
        };
        // SAFETY: buffers sized by OptiX for `input`; arguments per the forwarded contract.
        unsafe { Self::run(ctx, stream, &desc, input, args) }?;
        Ok(Arc::new(Self::make(ctx, input.kind(), buffer, handles, out_sizes, &opts.refs)))
    }

    /// Writes each object's exact output size (one `u32` per argument, `max_args` entries) for
    /// a later [`ClusterAccel::build_explicit`] (get-sizes mode).
    ///
    /// # Safety
    ///
    /// As [`ClusterAccel::build`] for the arguments; read the sizes on `stream` or after
    /// synchronizing it.
    pub unsafe fn get_sizes<T: ClusterArg>(ctx: &Arc<DeviceContext>, stream: &Arc<Stream>, scratch: &mut Scratch, input: &ClusterInput, args: &ClusterArgs<T>, budget: Option<usize>) -> Result<DeviceBuffer<u32>> {
        let n = Self::check_args(ctx, stream, input, args, &ClusterRefs::default())?;
        let sizes = Self::memory_usage(ctx, ClusterMode::GetSizes, input)?;
        within(budget, scratch.growth(sizes.temp).saturating_add(n.saturating_mul(size_of::<u32>())))?;
        let (temp, temp_len) = scratch.reserve(stream, sizes.temp)?;
        let out = zeroed_async::<u32>(stream, n)?;
        let mut desc = sys::OptixClusterAccelBuildModeDesc { mode: ClusterMode::GetSizes.raw(), ..Default::default() };
        desc.__bindgen_anon_1.getSize = sys::OptixClusterAccelBuildModeDescGetSize {
            outputSizesBuffer: out.cu_deviceptr(),
            outputSizesStrideInBytes: size_of::<u32>() as u32,
            tempBuffer: temp,
            tempBufferSizeInBytes: temp_len,
        };
        // SAFETY: sizes buffer holds `max_args` entries; temp sized by OptiX; arguments per the
        // forwarded contract.
        unsafe { Self::run(ctx, stream, &desc, input, args) }?;
        Ok(out)
    }

    /// Builds exactly `sizes.len()` (= `max_args`) objects packed at aligned offsets of one tight
    /// buffer (explicit destinations), from the sizes [`ClusterAccel::get_sizes`] reported.
    ///
    /// # Safety
    ///
    /// As [`ClusterAccel::build`]; additionally `sizes` must be the get-sizes result for these
    /// same arguments (OptiX writes that many bytes per object).
    pub unsafe fn build_explicit<T: ClusterArg>(
        ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        input: &ClusterInput,
        args: &ClusterArgs<T>,
        sizes: &[u32],
        opts: ClusterOptions<'_>,
    ) -> Result<Arc<Self>> {
        let n = Self::check_args(ctx, stream, input, args, &opts.refs)?;
        if args.count.is_some() {
            return Err(Error::Invalid("explicit builds build exactly sizes.len() arguments; give no device count"));
        }
        if sizes.len() != n {
            return Err(Error::Invalid("explicit builds need one size per argument (max_args)"));
        }
        let align = sys::OPTIX_ACCEL_BUFFER_BYTE_ALIGNMENT as usize;
        let mut offsets = Vec::with_capacity(n);
        let mut total = 0usize;
        for s in sizes {
            offsets.push(total);
            total = (*s as usize).checked_next_multiple_of(align).and_then(|s| total.checked_add(s)).ok_or(Error::Invalid("explicit cluster output size overflows usize"))?;
        }
        let temp_size = Self::memory_usage(ctx, ClusterMode::Explicit, input)?.temp;
        let arrays = n.saturating_mul(2 * size_of::<u64>()).saturating_add(if opts.sizes { n.saturating_mul(size_of::<u32>()) } else { 0 });
        within(opts.budget, scratch.growth(temp_size).saturating_add(total).saturating_add(arrays))?;
        let (temp, temp_len) = scratch.reserve(stream, temp_size)?;
        // SAFETY: OptiX writes every object before anything reads it.
        let buffer = unsafe { alloc(stream, total, sys::OPTIX_ACCEL_BUFFER_BYTE_ALIGNMENT) }?;
        // Each destination keeps the buffer's alignment: offsets are multiples of it.
        let dests: Vec<u64> = offsets.iter().map(|o| buffer.cu_deviceptr() + *o as u64).collect();
        // SAFETY: `u64` has no padding.
        let dest_buffer = unsafe { upload(stream, &dests) }?;
        let handles = zeroed_async::<u64>(stream, n)?;
        let out_sizes = if opts.sizes { Some(zeroed_async::<u32>(stream, n)?) } else { None };
        let mut desc = sys::OptixClusterAccelBuildModeDesc { mode: ClusterMode::Explicit.raw(), ..Default::default() };
        desc.__bindgen_anon_1.explicitDest = sys::OptixClusterAccelBuildModeDescExplicitDest {
            tempBuffer: temp,
            tempBufferSizeInBytes: temp_len,
            destAddressesBuffer: dest_buffer.cu_deviceptr(),
            destAddressesStrideInBytes: size_of::<u64>() as u32,
            outputHandlesBuffer: handles.cu_deviceptr(),
            outputHandlesStrideInBytes: size_of::<u64>() as u32,
            outputSizesBuffer: out_sizes.as_ref().map_or(0, |s| s.cu_deviceptr()),
            outputSizesStrideInBytes: if out_sizes.is_some() { size_of::<u32>() as u32 } else { 0 },
        };
        // SAFETY: each destination has its object's reported size (contract) inside `buffer`;
        // the address array is freed after the build in stream order; arguments per the
        // forwarded contract.
        unsafe { Self::run(ctx, stream, &desc, input, args) }?;
        drop(dest_buffer);
        Ok(Arc::new(Self::make(ctx, input.kind(), buffer, handles, out_sizes, &opts.refs)))
    }

    fn make(ctx: &Arc<DeviceContext>, kind: ClusterKind, buffer: DeviceBuffer<u8>, handles: DeviceBuffer<u64>, sizes: Option<DeviceBuffer<u32>>, refs: &ClusterRefs<'_>) -> Self {
        Self { kind, buffer, handles, sizes, _clusters: refs.clusters.to_vec(), _micromaps: refs.micromaps.to_vec(), ctx: ctx.clone() }
    }

    /// What was built.
    pub fn kind(&self) -> ClusterKind {
        self.kind
    }

    /// One entry per argument: CLAS / template device addresses, or GAS traversable handles
    /// (entries past a device count stay zero). Read on the build stream or after synchronizing.
    pub fn handles(&self) -> &DeviceBuffer<u64> {
        &self.handles
    }

    /// Per-object output sizes, if [`ClusterOptions::sizes`] was set.
    pub fn output_sizes(&self) -> Option<&DeviceBuffer<u32>> {
        self.sizes.as_ref()
    }

    /// Bytes of the output buffer.
    pub fn size_bytes(&self) -> usize {
        self.buffer.len()
    }

    /// The device context the objects were built on.
    pub fn context(&self) -> &Arc<DeviceContext> {
        &self.ctx
    }

    /// GAS `index` of a [`ClusterKind::GasesFromClusters`] build as an instance child: reads its
    /// handle back (synchronizes `stream`, which must be the build stream or ordered after it).
    pub fn child(self: &Arc<Self>, stream: &Stream, index: usize) -> Result<crate::Child> {
        if self.kind != ClusterKind::GasesFromClusters {
            return Err(Error::Invalid("only GasesFromClusters builds produce traversable GASes"));
        }
        let handles = self.handles.to_host_vec(stream)?;
        let handle = *handles.get(index).ok_or(Error::Invalid("cluster GAS index out of range"))?;
        if handle == 0 {
            return Err(Error::Invalid("cluster GAS slot was not built (past the device count)"));
        }
        Ok(crate::Child::Cluster(self.clone(), handle))
    }
}
