//! Acceleration structures: [`Accel`] over triangles, curves, spheres or custom primitives
//! (GAS), or over [`Instance`]s (IAS), plus post-build properties and relocation.
//!
//! **What.** A build sizes the structure (`optixAccelComputeMemoryUsage`), checks the caller's
//! byte budget, takes temp memory from a caller-owned [`Scratch`] (grown on demand, reused across
//! builds), allocates the output on the caller's stream, builds and, with
//! [`BuildOptions::compact`], reads the compacted size back and compacts into a tighter buffer.
//! Geometry inputs: [`TriangleMesh`] (every OptiX vertex and index format, per-primitive SBT
//! offsets, pre-transform, opacity micromaps), [`Curves`] (every curve type, rocaps, ribbons with
//! normals, end caps), [`Spheres`] (one or per-sphere radius) and [`CustomPrimitives`] (AABBs).
//! Instance inputs: an array of [`Instance`]s, or pointers into [`DeviceInstances`]. Motion blur
//! is [`BuildOptions::motion`] plus one buffer per key in every per-key input ([`Keys`]).
//! A structure built with [`BuildOptions::allow_update`] can be refitted in place (`refit_*`);
//! the refit must present the same shape (kinds, counts, formats, SBT records, motion keys), which
//! is checked against what the build recorded.
//! **Post-build.** [`Accel::emit`] writes the compacted size or the AABBs
//! (`optixAccelEmitProperty`); [`Accel::relocation_info`], [`RelocationInfo::compatible`] and
//! [`Accel::relocate`] copy a structure into a new buffer (possibly on another device context) and
//! patch it (`optixAccelRelocate`), relinking instance children or micromap arrays.
//! **Lifetimes.** Input buffers are only read during a build; the result references nothing but
//! its own buffer, except that an instance structure holds `Arc`s of the structures it instances
//! and a triangle structure holds `Arc`s of the opacity micromap arrays it uses.
//! **Ordering.** Output, compaction, instance and relocation buffers are allocated on the stream
//! and freed with `cuMemFreeAsync` on it, so frees are ordered after the work that uses them. A
//! [`Scratch`] or [`DeviceInstances`] belongs to one stream, which orders its reuse. Compaction
//! synchronizes the stream once.
//! **Used by** `micromap` (shared helpers, [`RelocationInfo`]), `cluster` ([`VertexFormat`],
//! [`Scratch`]), render-rs and the nv-rs OptiX tests.

use crate::micromap::{MicromapUse, OpacityMicromapArray};
use crate::xform::{Child, MotionOptions};
use crate::{DeviceContext, Error, Result, check, entry, sys};
use cuda_core::{DeviceBuffer, DeviceCopy, IntoResult, Stream};
use std::ffi::c_void;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

/// Per-input geometry flags (`OptixGeometryFlags`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GeometryFlags(pub u32);

impl GeometryFlags {
    /// Any-hit programs run as usual.
    pub const NONE: Self = Self(sys::OptixGeometryFlags_OPTIX_GEOMETRY_FLAG_NONE as u32);
    /// Opaque geometry: any-hit programs are skipped.
    pub const DISABLE_ANYHIT: Self =
        Self(sys::OptixGeometryFlags_OPTIX_GEOMETRY_FLAG_DISABLE_ANYHIT as u32);
    /// Any-hit runs at most once per primitive per ray.
    pub const REQUIRE_SINGLE_ANYHIT_CALL: Self =
        Self(sys::OptixGeometryFlags_OPTIX_GEOMETRY_FLAG_REQUIRE_SINGLE_ANYHIT_CALL as u32);
    /// Triangles are never culled by their facing.
    pub const DISABLE_TRIANGLE_FACE_CULLING: Self =
        Self(sys::OptixGeometryFlags_OPTIX_GEOMETRY_FLAG_DISABLE_TRIANGLE_FACE_CULLING as u32);
}

/// One SBT record with no flags: the default of [`TriangleMesh::new`] and friends.
const ONE_RECORD: &[GeometryFlags] = &[GeometryFlags::NONE];

/// Vertex position formats (`OptixVertexFormat`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VertexFormat {
    /// `[f32; 3]`.
    Float3,
    /// `[f32; 2]` (z = 0).
    Float2,
    /// Three IEEE binary16 values.
    Half3,
    /// Two IEEE binary16 values (z = 0).
    Half2,
    /// Three signed-normalized 16-bit values.
    Snorm16x3,
    /// Two signed-normalized 16-bit values (z = 0).
    Snorm16x2,
}

impl VertexFormat {
    pub(crate) fn raw(self) -> sys::OptixVertexFormat {
        match self {
            Self::Float3 => sys::OptixVertexFormat_OPTIX_VERTEX_FORMAT_FLOAT3,
            Self::Float2 => sys::OptixVertexFormat_OPTIX_VERTEX_FORMAT_FLOAT2,
            Self::Half3 => sys::OptixVertexFormat_OPTIX_VERTEX_FORMAT_HALF3,
            Self::Half2 => sys::OptixVertexFormat_OPTIX_VERTEX_FORMAT_HALF2,
            Self::Snorm16x3 => sys::OptixVertexFormat_OPTIX_VERTEX_FORMAT_SNORM16_3,
            Self::Snorm16x2 => sys::OptixVertexFormat_OPTIX_VERTEX_FORMAT_SNORM16_2,
        }
    }
}

/// A per-motion-key input: one buffer without motion, or one per key with
/// [`BuildOptions::motion`]. Every key's buffer must have the same length.
pub enum Keys<'a, T> {
    /// A single buffer (no motion).
    One(&'a DeviceBuffer<T>),
    /// One buffer per motion key, in key order.
    Motion(&'a [&'a DeviceBuffer<T>]),
}

impl<T> Clone for Keys<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Keys<'_, T> {}

impl<'a, T> From<&'a DeviceBuffer<T>> for Keys<'a, T> {
    fn from(buffer: &'a DeviceBuffer<T>) -> Self {
        Self::One(buffer)
    }
}

impl<T> Keys<'_, T> {
    fn buffers(&self) -> &[&DeviceBuffer<T>] {
        match self {
            Self::One(b) => std::slice::from_ref(b),
            Self::Motion(bs) => bs,
        }
    }

    /// Per-key device pointers and the element count every key shares.
    fn split(&self, keys: usize) -> Result<(Vec<sys::CUdeviceptr>, u32)> {
        let bufs = self.buffers();
        if bufs.len() != keys {
            return Err(Error::Invalid(
                "a per-key input needs exactly one buffer per motion key",
            ));
        }
        let len = bufs.first().map_or(0, |b| b.len());
        if bufs.iter().any(|b| b.len() != len) {
            return Err(Error::Invalid(
                "motion-key buffers of one input differ in length",
            ));
        }
        Ok((
            bufs.iter().map(|b| b.cu_deviceptr()).collect(),
            count(len, "too many elements in a build input")?,
        ))
    }
}

/// Triangle vertex positions in one of the OptiX formats, per motion key. Half values are raw
/// IEEE binary16 bits.
#[derive(Clone, Copy)]
pub enum Vertices<'a> {
    /// `OPTIX_VERTEX_FORMAT_FLOAT3`.
    Float3(Keys<'a, [f32; 3]>),
    /// `OPTIX_VERTEX_FORMAT_FLOAT2`.
    Float2(Keys<'a, [f32; 2]>),
    /// `OPTIX_VERTEX_FORMAT_HALF3`.
    Half3(Keys<'a, [u16; 3]>),
    /// `OPTIX_VERTEX_FORMAT_HALF2`.
    Half2(Keys<'a, [u16; 2]>),
    /// `OPTIX_VERTEX_FORMAT_SNORM16_3`.
    Snorm16x3(Keys<'a, [i16; 3]>),
    /// `OPTIX_VERTEX_FORMAT_SNORM16_2`.
    Snorm16x2(Keys<'a, [i16; 2]>),
}

impl<'a> From<&'a DeviceBuffer<[f32; 3]>> for Vertices<'a> {
    fn from(buffer: &'a DeviceBuffer<[f32; 3]>) -> Self {
        Self::Float3(Keys::One(buffer))
    }
}

/// Size of one element, as the stride OptiX is given (elements are a few bytes).
const fn stride<T>() -> u32 {
    size_of::<T>() as u32
}

impl Vertices<'_> {
    /// The format of every key.
    pub fn format(&self) -> VertexFormat {
        match self {
            Self::Float3(_) => VertexFormat::Float3,
            Self::Float2(_) => VertexFormat::Float2,
            Self::Half3(_) => VertexFormat::Half3,
            Self::Half2(_) => VertexFormat::Half2,
            Self::Snorm16x3(_) => VertexFormat::Snorm16x3,
            Self::Snorm16x2(_) => VertexFormat::Snorm16x2,
        }
    }

    /// Stride, per-key pointers and vertex count. Natural alignment (4 bytes for floats, 2 for the
    /// others) holds because every element starts at a multiple of its own size.
    fn split(&self, keys: usize) -> Result<(u32, Vec<sys::CUdeviceptr>, u32)> {
        let (stride, (ptrs, n)) = match self {
            Self::Float3(k) => (stride::<[f32; 3]>(), k.split(keys)?),
            Self::Float2(k) => (stride::<[f32; 2]>(), k.split(keys)?),
            Self::Half3(k) => (stride::<[u16; 3]>(), k.split(keys)?),
            Self::Half2(k) => (stride::<[u16; 2]>(), k.split(keys)?),
            Self::Snorm16x3(k) => (stride::<[i16; 3]>(), k.split(keys)?),
            Self::Snorm16x2(k) => (stride::<[i16; 2]>(), k.split(keys)?),
        };
        Ok((stride, ptrs, n))
    }
}

/// Triangle vertex-index triplets.
#[derive(Clone, Copy)]
pub enum Indices<'a> {
    /// `OPTIX_INDICES_FORMAT_UNSIGNED_BYTE3`.
    U8(&'a DeviceBuffer<[u8; 3]>),
    /// `OPTIX_INDICES_FORMAT_UNSIGNED_SHORT3`.
    U16(&'a DeviceBuffer<[u16; 3]>),
    /// `OPTIX_INDICES_FORMAT_UNSIGNED_INT3`.
    U32(&'a DeviceBuffer<[u32; 3]>),
}

impl Indices<'_> {
    /// Pointer, triangle count, format and stride.
    fn parts(&self) -> Result<(sys::CUdeviceptr, u32, sys::OptixIndicesFormat, u32)> {
        let (ptr, len, format, stride) = match self {
            Self::U8(b) => (
                b.cu_deviceptr(),
                b.len(),
                sys::OptixIndicesFormat_OPTIX_INDICES_FORMAT_UNSIGNED_BYTE3,
                stride::<[u8; 3]>(),
            ),
            Self::U16(b) => (
                b.cu_deviceptr(),
                b.len(),
                sys::OptixIndicesFormat_OPTIX_INDICES_FORMAT_UNSIGNED_SHORT3,
                stride::<[u16; 3]>(),
            ),
            Self::U32(b) => (
                b.cu_deviceptr(),
                b.len(),
                sys::OptixIndicesFormat_OPTIX_INDICES_FORMAT_UNSIGNED_INT3,
                stride::<[u32; 3]>(),
            ),
        };
        Ok((ptr, count(len, "too many triangles")?, format, stride))
    }
}

/// Per-primitive SBT record offsets (each in `0..flags.len()` of its input).
#[derive(Clone, Copy)]
pub enum SbtOffsets<'a> {
    /// 8-bit offsets.
    U8(&'a DeviceBuffer<u8>),
    /// 16-bit offsets.
    U16(&'a DeviceBuffer<u16>),
    /// 32-bit offsets.
    U32(&'a DeviceBuffer<u32>),
}

impl SbtOffsets<'_> {
    /// Pointer, element size and count.
    fn parts(&self) -> (sys::CUdeviceptr, u32, usize) {
        match self {
            Self::U8(b) => (b.cu_deviceptr(), stride::<u8>(), b.len()),
            Self::U16(b) => (b.cu_deviceptr(), stride::<u16>(), b.len()),
            Self::U32(b) => (b.cu_deviceptr(), stride::<u32>(), b.len()),
        }
    }
}

/// One triangle input of a geometry structure.
#[derive(Clone, Copy)]
pub struct TriangleMesh<'a> {
    /// Vertex positions per motion key.
    pub vertices: Vertices<'a>,
    /// Triangles as vertex-index triplets; `None` means consecutive vertex triplets.
    pub indices: Option<Indices<'a>>,
    /// Flags per SBT record; the length is the input's SBT record count (at least one).
    pub flags: &'a [GeometryFlags],
    /// Per-triangle SBT record offsets; required when `flags` has more than one record.
    pub sbt_offsets: Option<SbtOffsets<'a>>,
    /// Row-major 3x4 transform applied to the vertices at build time (one matrix).
    pub pre_transform: Option<&'a DeviceBuffer<[f32; 12]>>,
    /// Bias added to `optixGetPrimitiveIndex`.
    pub primitive_offset: u32,
    /// Opacity micromaps applied to the triangles.
    pub micromap: Option<MicromapUse<'a>>,
}

impl<'a> TriangleMesh<'a> {
    /// A mesh over `vertices` (non-indexed, one SBT record, no flags); set the other fields with
    /// struct-update syntax.
    pub fn new(vertices: impl Into<Vertices<'a>>) -> Self {
        Self {
            vertices: vertices.into(),
            indices: None,
            flags: ONE_RECORD,
            sbt_offsets: None,
            pre_transform: None,
            primitive_offset: 0,
            micromap: None,
        }
    }
}

/// Curve basis and cross-section (`OptixPrimitiveType` curve values).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurveType {
    /// Piecewise linear, round.
    RoundLinear,
    /// Quadratic B-spline, round.
    RoundQuadraticBspline,
    /// Cubic B-spline, round.
    RoundCubicBspline,
    /// Catmull-Rom, round.
    RoundCatmullRom,
    /// Cubic Bezier, round.
    RoundCubicBezier,
    /// Quadratic B-spline, round, rocaps intersector.
    RoundQuadraticBsplineRocaps,
    /// Cubic B-spline, round, rocaps intersector.
    RoundCubicBsplineRocaps,
    /// Catmull-Rom, round, rocaps intersector.
    RoundCatmullRomRocaps,
    /// Cubic Bezier, round, rocaps intersector.
    RoundCubicBezierRocaps,
    /// Quadratic B-spline with a flat, oriented cross-section (ribbon).
    FlatQuadraticBspline,
}

impl CurveType {
    fn raw(self) -> sys::OptixPrimitiveType {
        match self {
            Self::RoundLinear => sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_LINEAR,
            Self::RoundQuadraticBspline => {
                sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_QUADRATIC_BSPLINE
            }
            Self::RoundCubicBspline => {
                sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_CUBIC_BSPLINE
            }
            Self::RoundCatmullRom => sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_CATMULLROM,
            Self::RoundCubicBezier => {
                sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_CUBIC_BEZIER
            }
            Self::RoundQuadraticBsplineRocaps => {
                sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_QUADRATIC_BSPLINE_ROCAPS
            }
            Self::RoundCubicBsplineRocaps => {
                sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_CUBIC_BSPLINE_ROCAPS
            }
            Self::RoundCatmullRomRocaps => {
                sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_CATMULLROM_ROCAPS
            }
            Self::RoundCubicBezierRocaps => {
                sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_CUBIC_BEZIER_ROCAPS
            }
            Self::FlatQuadraticBspline => {
                sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_FLAT_QUADRATIC_BSPLINE
            }
        }
    }

    /// The `OptixPrimitiveTypeFlags` bit for [`crate::PipelineOptions::primitive_types`].
    pub fn flag(self) -> u32 {
        (match self {
            Self::RoundLinear => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_LINEAR,
            Self::RoundQuadraticBspline => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_QUADRATIC_BSPLINE,
            Self::RoundCubicBspline => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CUBIC_BSPLINE,
            Self::RoundCatmullRom => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CATMULLROM,
            Self::RoundCubicBezier => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CUBIC_BEZIER,
            Self::RoundQuadraticBsplineRocaps => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_QUADRATIC_BSPLINE_ROCAPS,
            Self::RoundCubicBsplineRocaps => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CUBIC_BSPLINE_ROCAPS,
            Self::RoundCatmullRomRocaps => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CATMULLROM_ROCAPS,
            Self::RoundCubicBezierRocaps => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CUBIC_BEZIER_ROCAPS,
            Self::FlatQuadraticBspline => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_FLAT_QUADRATIC_BSPLINE,
        }) as u32
    }

    /// Polynomial degree: a segment spans `degree + 1` consecutive vertices.
    fn degree(self) -> u32 {
        match self {
            Self::RoundLinear => 1,
            Self::RoundQuadraticBspline
            | Self::RoundQuadraticBsplineRocaps
            | Self::FlatQuadraticBspline => 2,
            Self::RoundCubicBspline
            | Self::RoundCatmullRom
            | Self::RoundCubicBezier
            | Self::RoundCubicBsplineRocaps
            | Self::RoundCatmullRomRocaps
            | Self::RoundCubicBezierRocaps => 3,
        }
    }
}

/// One curve input of a geometry structure (one SBT record per input).
#[derive(Clone, Copy)]
pub struct Curves<'a> {
    /// Basis and cross-section.
    pub kind: CurveType,
    /// Control points per motion key.
    pub vertices: Keys<'a, [f32; 3]>,
    /// Radius per control point, per motion key.
    pub widths: Keys<'a, f32>,
    /// Ribbon normals per control point, per motion key; only for
    /// [`CurveType::FlatQuadraticBspline`] (the SDK header marks the field reserved, so the
    /// driver may reject it).
    pub normals: Option<Keys<'a, [f32; 3]>>,
    /// First control point of each segment (one primitive per segment).
    pub segments: &'a DeviceBuffer<u32>,
    /// Geometry flags.
    pub flags: GeometryFlags,
    /// Flat end caps on quadratic/cubic segments (not valid for linear curves).
    pub endcaps: bool,
    /// Bias added to `optixGetPrimitiveIndex`.
    pub primitive_offset: u32,
}

/// Sphere radii: one per key buffer, or one per sphere.
#[derive(Clone, Copy)]
pub enum Radii<'a> {
    /// Every sphere has the single value of its key buffer.
    Single(Keys<'a, f32>),
    /// One radius per sphere, per motion key.
    PerSphere(Keys<'a, f32>),
}

/// One sphere input of a geometry structure.
#[derive(Clone, Copy)]
pub struct Spheres<'a> {
    /// Centers per motion key.
    pub centers: Keys<'a, [f32; 3]>,
    /// Radii.
    pub radii: Radii<'a>,
    /// Flags per SBT record (at least one).
    pub flags: &'a [GeometryFlags],
    /// Per-sphere SBT record offsets; required when `flags` has more than one record.
    pub sbt_offsets: Option<SbtOffsets<'a>>,
    /// Bias added to `optixGetPrimitiveIndex`.
    pub primitive_offset: u32,
}

impl<'a> Spheres<'a> {
    /// Spheres without motion, one SBT record, no flags.
    pub fn new(centers: &'a DeviceBuffer<[f32; 3]>, radii: Radii<'a>) -> Self {
        Self {
            centers: Keys::One(centers),
            radii,
            flags: ONE_RECORD,
            sbt_offsets: None,
            primitive_offset: 0,
        }
    }
}

/// An axis-aligned box, layout-identical to `OptixAabb`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Aabb {
    /// Lower corner.
    pub min: [f32; 3],
    /// Upper corner.
    pub max: [f32; 3],
}

// SAFETY: `repr(C)` plain `f32` arrays: no padding, every bit pattern valid, no ownership.
unsafe impl DeviceCopy for Aabb {}

const _: () = assert!(size_of::<Aabb>() == size_of::<sys::OptixAabb>());
const _: () =
    assert!(std::mem::offset_of!(Aabb, max) == std::mem::offset_of!(sys::OptixAabb, maxX));
// The tight stride OptiX is given must keep every box on the AABB buffer alignment.
const _: () =
    assert!(size_of::<Aabb>().is_multiple_of(sys::OPTIX_AABB_BUFFER_BYTE_ALIGNMENT as usize));

/// One custom-primitive input of a geometry structure; intersection programs decide hits.
#[derive(Clone, Copy)]
pub struct CustomPrimitives<'a> {
    /// One box per primitive, per motion key.
    pub aabbs: Keys<'a, Aabb>,
    /// Flags per SBT record (at least one).
    pub flags: &'a [GeometryFlags],
    /// Per-primitive SBT record offsets; required when `flags` has more than one record.
    pub sbt_offsets: Option<SbtOffsets<'a>>,
    /// Bias added to `optixGetPrimitiveIndex`.
    pub primitive_offset: u32,
}

impl<'a> CustomPrimitives<'a> {
    /// Primitives without motion, one SBT record, no flags.
    pub fn new(aabbs: &'a DeviceBuffer<Aabb>) -> Self {
        Self {
            aabbs: Keys::One(aabbs),
            flags: ONE_RECORD,
            sbt_offsets: None,
            primitive_offset: 0,
        }
    }
}

/// How a structure is built.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BuildOptions {
    /// Favour trace speed over build speed.
    pub prefer_fast_trace: bool,
    /// Compact after building (one stream synchronization, usually halves the memory).
    pub compact: bool,
    /// Allow `refit_*` later (costs some trace speed and memory).
    pub allow_update: bool,
    /// New device bytes the build may allocate at peak (scratch growth + output); `None` admits
    /// any size. Checked after OptiX reports the sizes and before anything is allocated.
    pub budget: Option<usize>,
    /// Motion keys per input (1 = static; every per-key input then carries this many buffers).
    pub motion_keys: u16,
    /// Time range and vanish flags of the keys (ignored when `motion_keys` is 1).
    pub motion: MotionOptions,
    /// Allow `optixGet*VertexDataFromHandle` / `optixGetSphereDataFromHandle` on the result.
    pub random_vertex_access: bool,
    /// Allow `optixGetInstanceTraversableFromIAS` on the result.
    pub random_instance_access: bool,
    /// Refits may swap opacity micromap arrays and indices.
    pub allow_micromap_update: bool,
    /// Instances may disable this structure's opacity micromaps.
    pub allow_disable_micromaps: bool,
}

impl Default for BuildOptions {
    /// Fast trace, compacted, static, no budget: the right choice for static geometry.
    fn default() -> Self {
        Self {
            prefer_fast_trace: true,
            compact: true,
            allow_update: false,
            budget: None,
            motion_keys: 1,
            motion: MotionOptions::default(),
            random_vertex_access: false,
            random_instance_access: false,
            allow_micromap_update: false,
            allow_disable_micromaps: false,
        }
    }
}

impl BuildOptions {
    fn flags(&self) -> u32 {
        let mut flags = if self.prefer_fast_trace {
            sys::OptixBuildFlags_OPTIX_BUILD_FLAG_PREFER_FAST_TRACE
        } else {
            sys::OptixBuildFlags_OPTIX_BUILD_FLAG_PREFER_FAST_BUILD
        } as u32;
        for (on, flag) in [
            (
                self.compact,
                sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_COMPACTION,
            ),
            (
                self.allow_update,
                sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_UPDATE,
            ),
            (
                self.random_vertex_access,
                sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_RANDOM_VERTEX_ACCESS,
            ),
            (
                self.random_instance_access,
                sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_RANDOM_INSTANCE_ACCESS,
            ),
            (
                self.allow_micromap_update,
                sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_OPACITY_MICROMAP_UPDATE,
            ),
            (
                self.allow_disable_micromaps,
                sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_DISABLE_OPACITY_MICROMAPS,
            ),
        ] {
            if on {
                flags |= flag as u32;
            }
        }
        flags
    }

    fn motion_sys(&self) -> Result<sys::OptixMotionOptions> {
        match self.motion_keys {
            0 => Err(Error::Invalid("motion_keys must be at least 1")),
            1 => Ok(sys::OptixMotionOptions::default()),
            keys => self.motion.to_sys(keys),
        }
    }
}

/// `n` as the `u32` OptiX counts in, or `what` as the error.
pub(crate) fn count(n: usize, what: &'static str) -> Result<u32> {
    u32::try_from(n).map_err(|_| Error::Invalid(what))
}

/// Rejects a stream of another CUDA context than `ctx`'s.
pub(crate) fn same_ctx(stream: &Stream, ctx: &DeviceContext) -> Result<()> {
    if stream.context().as_ref() != ctx.cuda().as_ref() {
        return Err(Error::Invalid(
            "stream belongs to another CUDA context than the OptiX device context",
        ));
    }
    Ok(())
}

/// Rejects a peak allocation above the caller's budget.
pub(crate) fn within(budget: Option<usize>, peak: usize) -> Result<()> {
    match budget {
        Some(available) if peak > available => Err(Error::Budget {
            requested: peak,
            available,
        }),
        _ => Ok(()),
    }
}

/// `bytes` (at least one) of stream-ordered device memory whose address is a multiple of
/// `align`.
///
/// # Safety
///
/// The contents are uninitialized: the caller has them written (by OptiX or a copy) before
/// anything reads them.
pub(crate) unsafe fn alloc(
    stream: &Arc<Stream>,
    bytes: usize,
    align: u32,
) -> Result<DeviceBuffer<u8>> {
    // SAFETY: forwarded contract.
    let buffer = unsafe { DeviceBuffer::<u8>::uninitialized_async(stream, bytes.max(1)) }?;
    if buffer.cu_deviceptr() % u64::from(align) != 0 {
        return Err(Error::Invalid(
            "allocator returned memory below the alignment OptiX requires",
        ));
    }
    Ok(buffer)
}

/// Copies `items` into a fresh stream-ordered device buffer (at least one byte).
///
/// # Safety
///
/// `T` has no padding bytes, so every copied byte is initialized.
pub(crate) unsafe fn upload<T: Copy>(
    stream: &Arc<Stream>,
    items: &[T],
) -> Result<DeviceBuffer<u8>> {
    let bytes = size_of_val(items);
    // SAFETY: filled by the copy below before anything reads it.
    let device = unsafe { DeviceBuffer::<u8>::uninitialized_async(stream, bytes.max(1)) }?;
    if bytes > 0 {
        // SAFETY: `items` is `bytes` initialized bytes (no padding, per the contract); a pageable
        // source is staged before the call returns, so `items` may be dropped afterwards; the
        // destination holds at least `bytes` bytes.
        unsafe {
            cuda_core::sys::cuMemcpyHtoDAsync_v2(
                device.cu_deviceptr(),
                items.as_ptr().cast(),
                bytes,
                stream.cu_stream(),
            )
        }
        .result()?;
    }
    Ok(device)
}

/// Copies all of `src` into `dst` on `stream`, across device contexts if they differ.
///
/// # Safety
///
/// Every write to `src` is complete or ordered before `stream`; `dst` is not in use.
pub(crate) unsafe fn copy_bytes(
    stream: &Stream,
    dst: &DeviceBuffer<u8>,
    src: &DeviceBuffer<u8>,
) -> Result<()> {
    if dst.len() != src.len() {
        return Err(Error::Invalid(
            "relocation target size differs from the source",
        ));
    }
    stream.context().bind_to_thread()?;
    if dst.context().as_ref() == src.context().as_ref() {
        // SAFETY: both hold `len` bytes in one context; distinct allocations do not overlap;
        // ordering per the contract.
        unsafe {
            cuda_core::sys::cuMemcpyDtoDAsync_v2(
                dst.cu_deviceptr(),
                src.cu_deviceptr(),
                src.len(),
                stream.cu_stream(),
            )
        }
        .result()?;
    } else {
        // SAFETY: as above, each pointer paired with the context that owns it.
        unsafe {
            cuda_core::sys::cuMemcpyPeerAsync(
                dst.cu_deviceptr(),
                dst.context().cu_ctx(),
                src.cu_deviceptr(),
                src.context().cu_ctx(),
                src.len(),
                stream.cu_stream(),
            )
        }
        .result()?;
    }
    Ok(())
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
        Self {
            stream: stream.clone(),
            buffer: None,
        }
    }

    /// Bytes currently held.
    pub fn capacity(&self) -> usize {
        self.buffer.as_ref().map_or(0, |b| b.len())
    }

    /// Bytes a request for `bytes` would newly allocate.
    pub(crate) fn growth(&self, bytes: usize) -> usize {
        bytes.saturating_sub(self.capacity())
    }

    /// Device memory of at least `bytes` (at least one byte, so the pointer is never null),
    /// aligned for every OptiX temp buffer.
    pub(crate) fn reserve(
        &mut self,
        stream: &Arc<Stream>,
        bytes: usize,
    ) -> Result<(sys::CUdeviceptr, usize)> {
        if !Arc::ptr_eq(stream, &self.stream) {
            return Err(Error::Invalid(
                "scratch used on a different stream than it was created for",
            ));
        }
        let bytes = bytes.max(1);
        if self.capacity() < bytes {
            // The old buffer's free is ordered after the work already queued on this stream.
            self.buffer = None;
            // SAFETY: OptiX writes temp memory before reading it.
            self.buffer =
                Some(unsafe { alloc(stream, bytes, sys::OPTIX_ACCEL_BUFFER_BYTE_ALIGNMENT) }?);
        }
        let b = self
            .buffer
            .as_ref()
            .ok_or(Error::Invalid("scratch buffer missing after reserve"))?;
        Ok((b.cu_deviceptr(), b.len()))
    }
}

/// One instance of a structure inside an instance structure.
#[derive(Clone)]
pub struct Instance {
    /// What is instanced: a structure, a transform (motion blur) or a cluster GAS.
    pub child: Child,
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

    /// `child` with the identity transform, id 0, SBT offset 0, visible to every ray.
    pub fn new(child: impl Into<Child>) -> Self {
        Self {
            child: child.into(),
            transform: Self::IDENTITY,
            id: 0,
            sbt_offset: 0,
            mask: 0xFF,
            flags: 0,
        }
    }

    fn raw(&self) -> sys::OptixInstance {
        sys::OptixInstance {
            transform: self.transform,
            instanceId: self.id,
            sbtOffset: self.sbt_offset,
            visibilityMask: u32::from(self.mask),
            flags: self.flags,
            traversableHandle: self.child.handle(),
            pad: [0; 2],
        }
    }
}

// 12 f32 + 4 u32 + u64 + 2 u32 = 80 bytes: no padding, so instance arrays upload byte-exact.
const _: () = assert!(size_of::<sys::OptixInstance>() == 80);
// Consecutive instances (and pointers into an array of them) stay on the instance alignment.
const _: () = assert!(
    size_of::<sys::OptixInstance>().is_multiple_of(sys::OPTIX_INSTANCE_BYTE_ALIGNMENT as usize)
);

/// Rejects instances of structures from another device context.
fn check_children(ctx: &Arc<DeviceContext>, instances: &[Instance]) -> Result<()> {
    if instances
        .iter()
        .any(|i| !Arc::ptr_eq(i.child.context(), ctx))
    {
        return Err(Error::Invalid(
            "instance of a structure from another device context",
        ));
    }
    Ok(())
}

/// Instances resident on the device, for instance structures built over pointers to them
/// (`OPTIX_BUILD_INPUT_TYPE_INSTANCE_POINTERS`); one set can feed several structures.
pub struct DeviceInstances {
    buffer: DeviceBuffer<u8>,
    children: Vec<Child>,
    stream: Arc<Stream>,
    ctx: Arc<DeviceContext>,
}

impl DeviceInstances {
    /// Uploads `instances` on `stream`; builds that point into them must run on `stream`.
    pub fn new(
        ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        instances: &[Instance],
    ) -> Result<Self> {
        same_ctx(stream, ctx)?;
        if instances.is_empty() {
            return Err(Error::Invalid(
                "device instances need at least one instance",
            ));
        }
        check_children(ctx, instances)?;
        let raw: Vec<sys::OptixInstance> = instances.iter().map(Instance::raw).collect();
        // SAFETY: `OptixInstance` has no padding (asserted above).
        let buffer = unsafe { upload(stream, &raw) }?;
        if buffer.cu_deviceptr() % u64::from(sys::OPTIX_INSTANCE_BYTE_ALIGNMENT) != 0 {
            return Err(Error::Invalid(
                "allocator returned instances below OptiX's instance alignment",
            ));
        }
        Ok(Self {
            buffer,
            children: instances.iter().map(|i| i.child.clone()).collect(),
            stream: stream.clone(),
            ctx: ctx.clone(),
        })
    }

    /// Number of instances.
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// Always false: a set holds at least one instance.
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}

/// Default pinned host staging capacity (`256 KiB` = 3,276 `OptixInstance` records) for
/// asynchronous H2D DMA transfers in [`InstanceBuffer`].
pub const INSTANCE_STAGING_BYTES: usize = 256 * 1024;

/// Reusable page-locked host staging buffer (`cuMemAllocHost_v2` / `cuMemFreeHost`).
struct PinnedHostStaging {
    ptr: *mut c_void,
    bytes: usize,
    ctx: Arc<cuda_core::Context>,
}

// SAFETY: page-locked CUDA host memory is context-owned and not bound to a single OS thread;
// mutable access is exclusively serialized through `&mut InstanceBuffer`.
unsafe impl Send for PinnedHostStaging {}

impl PinnedHostStaging {
    fn try_new(ctx: &Arc<cuda_core::Context>, bytes: usize) -> Option<Self> {
        if bytes == 0 || ctx.bind_to_thread().is_err() {
            return None;
        }
        let mut ptr: *mut c_void = std::ptr::null_mut();
        // SAFETY: `ctx` is current on this thread and `ptr` is a valid out-pointer.
        let res = unsafe { cuda_core::sys::cuMemAllocHost_v2(&mut ptr, bytes) };
        if res == cuda_core::sys::cudaError_enum_CUDA_SUCCESS && !ptr.is_null() {
            Some(Self {
                ptr,
                bytes,
                ctx: ctx.clone(),
            })
        } else {
            None
        }
    }
}

impl Drop for PinnedHostStaging {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.ctx.bind_to_thread().is_ok() {
            // SAFETY: `self.ptr` was allocated by `cuMemAllocHost_v2` on `self.ctx`.
            unsafe {
                let _ = cuda_core::sys::cuMemFreeHost(self.ptr);
            }
            self.ptr = std::ptr::null_mut();
        }
    }
}

/// Persistent, grow-only device `OptixInstance` array paired with a reusable `256 KiB` pinned
/// host staging buffer for zero-churn per-frame IAS builds and partial dynamic updates.
pub struct InstanceBuffer {
    ctx: Arc<DeviceContext>,
    stream: Arc<Stream>,
    device: Option<DeviceBuffer<u8>>,
    cap_instances: usize,
    len_instances: usize,
    raw_scratch: Vec<sys::OptixInstance>,
    children: Vec<Child>,
    pinned: Option<PinnedHostStaging>,
}

impl InstanceBuffer {
    /// Creates an empty persistent instance buffer bound to `ctx` and `stream`, pre-allocating
    /// a `256 KiB` page-locked host staging buffer for async DMA uploads.
    pub fn new(ctx: &Arc<DeviceContext>, stream: &Arc<Stream>) -> Self {
        let pinned = PinnedHostStaging::try_new(ctx.cuda(), INSTANCE_STAGING_BYTES);
        Self {
            ctx: ctx.clone(),
            stream: stream.clone(),
            device: None,
            cap_instances: 0,
            len_instances: 0,
            raw_scratch: Vec::new(),
            children: Vec::new(),
            pinned,
        }
    }

    /// Number of active instances currently resident in the device buffer.
    pub fn len(&self) -> usize {
        self.len_instances
    }

    /// Returns `true` when zero instances are uploaded.
    pub fn is_empty(&self) -> bool {
        self.len_instances == 0
    }

    /// Base `CUdeviceptr` of the persistent `OptixInstance` device array (`0` if unallocated).
    pub fn device_ptr(&self) -> u64 {
        self.device.as_ref().map_or(0, DeviceBuffer::cu_deviceptr)
    }

    /// Allocated device buffer capacity in bytes.
    pub fn capacity_bytes(&self) -> usize {
        self.device.as_ref().map_or(0, DeviceBuffer::len)
    }

    /// Uploads a full slice of `instances`, growing the persistent device buffer only when
    /// `instances.len()` exceeds capacity (**zero** VRAM allocations/frees in steady state).
    pub fn upload_all(&mut self, stream: &Arc<Stream>, instances: &[Instance]) -> Result<()> {
        same_ctx(stream, &self.ctx)?;
        if !Arc::ptr_eq(stream, &self.stream) {
            return Err(Error::Invalid(
                "instance buffer used on a different stream than it was created for",
            ));
        }
        if instances.is_empty() {
            return Err(Error::Invalid(
                "an instance structure needs at least one instance",
            ));
        }
        check_children(&self.ctx, instances)?;
        let _ = count(instances.len(), "too many instances")?;

        if instances.len() > self.cap_instances {
            let new_cap = instances
                .len()
                .max(self.cap_instances.saturating_mul(2))
                .max(64);
            let new_bytes = new_cap
                .checked_mul(size_of::<sys::OptixInstance>())
                .ok_or(Error::Invalid("instance buffer byte size overflow"))?;
            // Drop old buffer before allocating so stream-ordered async free precedes reuse.
            self.device = None;
            // SAFETY: overwritten by the H2D copy below before OptiX reads it.
            let buf = unsafe { alloc(stream, new_bytes, sys::OPTIX_INSTANCE_BYTE_ALIGNMENT) }?;
            self.device = Some(buf);
            self.cap_instances = new_cap;
        }

        self.raw_scratch.clear();
        self.raw_scratch.reserve(instances.len());
        self.children.clear();
        self.children.reserve(instances.len());
        for inst in instances {
            self.raw_scratch.push(inst.raw());
            self.children.push(inst.child.clone());
        }
        self.len_instances = instances.len();

        let dst = self.device_ptr();
        self.copy_raw_range_to_device(stream, dst, 0)?;
        Ok(())
    }

    /// Updates a contiguous subrange `[offset .. offset + instances.len()]` in place via the
    /// reusable `256 KiB` pinned host staging buffer (**zero** VRAM allocations, **zero** frees,
    /// and **zero** host `Vec` allocations once warm).
    pub fn update_prefix(
        &mut self,
        stream: &Arc<Stream>,
        offset: usize,
        instances: &[Instance],
    ) -> Result<()> {
        same_ctx(stream, &self.ctx)?;
        if !Arc::ptr_eq(stream, &self.stream) {
            return Err(Error::Invalid(
                "instance buffer used on a different stream than it was created for",
            ));
        }
        if instances.is_empty() {
            return Ok(());
        }
        let end = offset
            .checked_add(instances.len())
            .ok_or(Error::Invalid("instance update range overflow"))?;
        if end > self.len_instances {
            return Err(Error::Invalid(
                "instance prefix update exceeds uploaded instance count",
            ));
        }
        check_children(&self.ctx, instances)?;

        self.raw_scratch.clear();
        self.raw_scratch.reserve(instances.len());
        for (i, inst) in instances.iter().enumerate() {
            self.raw_scratch.push(inst.raw());
            if let Some(slot) = self.children.get_mut(offset + i) {
                *slot = inst.child.clone();
            }
        }

        let base_ptr = self.device_ptr();
        let byte_offset = (offset * size_of::<sys::OptixInstance>()) as u64;
        self.copy_raw_range_to_device(stream, base_ptr, byte_offset)?;
        Ok(())
    }

    /// Streams `self.raw_scratch` to `base_ptr + byte_offset`, using the pinned `256 KiB` host
    /// staging buffer when the payload fits in one async DMA transfer (or direct H2D otherwise).
    fn copy_raw_range_to_device(
        &self,
        stream: &Arc<Stream>,
        base_ptr: u64,
        byte_offset: u64,
    ) -> Result<()> {
        let total_bytes = size_of_val(self.raw_scratch.as_slice());
        if total_bytes == 0 {
            return Ok(());
        }
        let dst_ptr = base_ptr + byte_offset;
        if let Some(ref pinned) = self.pinned
            && total_bytes <= pinned.bytes
        {
            // SAFETY: `OptixInstance` is 80 B with no padding (const-asserted above);
            // `pinned.ptr` holds `pinned.bytes >= total_bytes` page-locked host bytes;
            // `dst_ptr` points within `self.device` with at least `total_bytes` remaining.
            unsafe {
                std::ptr::copy_nonoverlapping(
                    self.raw_scratch.as_ptr().cast::<u8>(),
                    pinned.ptr.cast::<u8>(),
                    total_bytes,
                );
                cuda_core::sys::cuMemcpyHtoDAsync_v2(
                    dst_ptr,
                    pinned.ptr,
                    total_bytes,
                    stream.cu_stream(),
                )
            }
            .result()?;
            return Ok(());
        }
        // SAFETY: `OptixInstance` has no padding; CUDA stages pageable host sources synchronously
        // before returning from `cuMemcpyHtoDAsync_v2`; `dst_ptr` holds `>= total_bytes` bytes.
        unsafe {
            cuda_core::sys::cuMemcpyHtoDAsync_v2(
                dst_ptr,
                self.raw_scratch.as_ptr().cast(),
                total_bytes,
                stream.cu_stream(),
            )
        }
        .result()?;
        Ok(())
    }
}

/// What one build input was, recorded so refits and relocations can be checked against it.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Item {
    /// `omm`: the micromap array's address, or 0 when refits may swap it; `None` = no micromap.
    Tri {
        vertices: u32,
        triangles: u32,
        format: sys::OptixVertexFormat,
        index: sys::OptixIndicesFormat,
        sbt: u32,
        omm: Option<u64>,
    },
    Curve {
        kind: sys::OptixPrimitiveType,
        segments: u32,
        vertices: u32,
        normals: bool,
    },
    Sphere {
        spheres: u32,
        single: bool,
        sbt: u32,
    },
    Custom {
        prims: u32,
        sbt: u32,
    },
    Instances(u32),
    InstancePointers(u32),
}

/// What a structure keeps alive: instanced structures and micromap arrays it reads.
#[derive(Clone, Default)]
struct Refs {
    children: Vec<Child>,
    micromaps: Vec<Arc<OpacityMicromapArray>>,
}

/// Build inputs plus the host arrays and device buffers their pointers refer to. Inner `Vec`s
/// keep their heap storage when the outer ones grow, so recorded pointers stay valid.
#[derive(Default)]
struct Inputs {
    inputs: Vec<sys::OptixBuildInput>,
    shape: Vec<Item>,
    ptrs: Vec<Vec<sys::CUdeviceptr>>,
    flags: Vec<Vec<u32>>,
    usage: Vec<Vec<sys::OptixOpacityMicromapUsageCount>>,
    // Held, never read: device arrays (instances) the build reads through `inputs`.
    _device: Vec<DeviceBuffer<u8>>,
    refs: Refs,
}

impl Inputs {
    fn keep_ptrs(&mut self, v: Vec<sys::CUdeviceptr>) -> *const sys::CUdeviceptr {
        let p = v.as_ptr();
        self.ptrs.push(v);
        p
    }

    fn keep_usage(
        &mut self,
        v: Vec<sys::OptixOpacityMicromapUsageCount>,
    ) -> *const sys::OptixOpacityMicromapUsageCount {
        if v.is_empty() {
            return std::ptr::null();
        }
        let p = v.as_ptr();
        self.usage.push(v);
        p
    }

    /// Per-SBT-record flags and the optional per-primitive offsets of one input over `prims`
    /// primitives: (flags, records, offsets, offset size, offset stride).
    fn sbt(
        &mut self,
        flags: &[GeometryFlags],
        offsets: Option<SbtOffsets<'_>>,
        prims: u32,
    ) -> Result<(*const u32, u32, sys::CUdeviceptr, u32, u32)> {
        if flags.is_empty() {
            return Err(Error::Invalid(
                "a build input needs at least one SBT record (flags entry)",
            ));
        }
        let records = count(flags.len(), "too many SBT records")?;
        let (ptr, size, stride) = match offsets {
            Some(o) => {
                let (ptr, size, len) = o.parts();
                if len != prims as usize {
                    return Err(Error::Invalid(
                        "SBT offset buffer length differs from the primitive count",
                    ));
                }
                (ptr, size, size)
            }
            None if records > 1 => {
                return Err(Error::Invalid(
                    "several SBT records need per-primitive SBT offsets",
                ));
            }
            None => (0, 0, 0),
        };
        let raw: Vec<u32> = flags.iter().map(|f| f.0).collect();
        let p = raw.as_ptr();
        self.flags.push(raw);
        Ok((p, records, ptr, size, stride))
    }
}

/// What every input of one build shares.
struct Cx<'a> {
    ctx: &'a Arc<DeviceContext>,
    stream: &'a Arc<Stream>,
    /// Buffers per per-key input (motion keys, or 1).
    keys: usize,
    /// The structure allows micromap updates, so refits need not repeat the array.
    omm_update: bool,
}

/// Rejects `primitive_offset + prims` overflowing `u32`, which OptiX forbids.
fn prim_range(prims: u32, offset: u32) -> Result<()> {
    prims.checked_add(offset).map(|_| ()).ok_or(Error::Invalid(
        "primitive_offset + primitive count overflows u32",
    ))
}

/// A geometry build input kind; all inputs of one geometry structure share the kind.
trait Gas {
    fn push(&self, cx: &Cx<'_>, out: &mut Inputs) -> Result<()>;
}

impl Gas for TriangleMesh<'_> {
    fn push(&self, cx: &Cx<'_>, out: &mut Inputs) -> Result<()> {
        let (vstride, vptrs, vertices) = self.vertices.split(cx.keys)?;
        let (index_ptr, triplets, index_format, index_stride) = match self.indices {
            Some(ix) => ix.parts()?,
            None => (0, 0, sys::OptixIndicesFormat_OPTIX_INDICES_FORMAT_NONE, 0),
        };
        let triangles = match self.indices {
            Some(_) => triplets,
            None if vertices % 3 != 0 => {
                return Err(Error::Invalid(
                    "a non-indexed mesh needs a multiple of three vertices",
                ));
            }
            None => vertices / 3,
        };
        prim_range(triangles, self.primitive_offset)?;
        let (flags, records, sbt_ptr, sbt_size, sbt_stride) =
            out.sbt(self.flags, self.sbt_offsets, triangles)?;
        let (pre_transform, transform_format) = match self.pre_transform {
            Some(t) if t.len() != 1 => {
                return Err(Error::Invalid("pre_transform holds exactly one 3x4 matrix"));
            }
            Some(t)
                if t.cu_deviceptr() % u64::from(sys::OPTIX_GEOMETRY_TRANSFORM_BYTE_ALIGNMENT)
                    != 0 =>
            {
                return Err(Error::Invalid(
                    "pre_transform is below OptiX's geometry transform alignment",
                ));
            }
            Some(t) => (
                t.cu_deviceptr(),
                sys::OptixTransformFormat_OPTIX_TRANSFORM_FORMAT_MATRIX_FLOAT12,
            ),
            None => (0, sys::OptixTransformFormat_OPTIX_TRANSFORM_FORMAT_NONE),
        };
        let (opacity_micromap, omm) = match &self.micromap {
            Some(m) => {
                let (mut raw, usage) = m.input(cx.ctx, triangles)?;
                raw.micromapUsageCounts = out.keep_usage(usage);
                out.refs.micromaps.push(m.array().clone());
                (raw, Some(if cx.omm_update { 0 } else { m.array().ptr() }))
            }
            None => (sys::OptixBuildInputOpacityMicromap::default(), None),
        };
        let vertex_buffers = out.keep_ptrs(vptrs);
        let mut input = sys::OptixBuildInput {
            type_: sys::OptixBuildInputType_OPTIX_BUILD_INPUT_TYPE_TRIANGLES,
            ..Default::default()
        };
        input.__bindgen_anon_1.triangleArray = sys::OptixBuildInputTriangleArray {
            vertexBuffers: vertex_buffers,
            numVertices: vertices,
            vertexFormat: self.vertices.format().raw(),
            vertexStrideInBytes: vstride,
            indexBuffer: index_ptr,
            numIndexTriplets: triplets,
            indexFormat: index_format,
            indexStrideInBytes: index_stride,
            preTransform: pre_transform,
            flags,
            numSbtRecords: records,
            sbtIndexOffsetBuffer: sbt_ptr,
            sbtIndexOffsetSizeInBytes: sbt_size,
            sbtIndexOffsetStrideInBytes: sbt_stride,
            primitiveIndexOffset: self.primitive_offset,
            transformFormat: transform_format,
            opacityMicromap: opacity_micromap,
        };
        out.inputs.push(input);
        out.shape.push(Item::Tri {
            vertices,
            triangles,
            format: self.vertices.format().raw(),
            index: index_format,
            sbt: records,
            omm,
        });
        Ok(())
    }
}

impl Gas for Curves<'_> {
    fn push(&self, cx: &Cx<'_>, out: &mut Inputs) -> Result<()> {
        let (vptrs, vertices) = self.vertices.split(cx.keys)?;
        let (wptrs, widths) = self.widths.split(cx.keys)?;
        if widths != vertices {
            return Err(Error::Invalid("curves need one width per control point"));
        }
        let normals = match self.normals {
            Some(_) if self.kind != CurveType::FlatQuadraticBspline => {
                return Err(Error::Invalid(
                    "curve normals apply to flat quadratic B-spline ribbons only",
                ));
            }
            Some(n) => {
                let (nptrs, normals) = n.split(cx.keys)?;
                if normals != vertices {
                    return Err(Error::Invalid("ribbons need one normal per control point"));
                }
                Some(nptrs)
            }
            None => None,
        };
        if self.endcaps && self.kind == CurveType::RoundLinear {
            return Err(Error::Invalid("end caps are not valid for linear curves"));
        }
        let segments = count(self.segments.len(), "too many curve segments")?;
        if segments == 0 {
            return Err(Error::Invalid("curves need at least one segment"));
        }
        if vertices <= self.kind.degree() {
            return Err(Error::Invalid(
                "curves need more control points than their degree",
            ));
        }
        prim_range(segments, self.primitive_offset)?;
        let has_normals = normals.is_some();
        let vertex_buffers = out.keep_ptrs(vptrs);
        let width_buffers = out.keep_ptrs(wptrs);
        let normal_buffers = normals.map_or(std::ptr::null(), |n| out.keep_ptrs(n));
        let mut input = sys::OptixBuildInput {
            type_: sys::OptixBuildInputType_OPTIX_BUILD_INPUT_TYPE_CURVES,
            ..Default::default()
        };
        input.__bindgen_anon_1.curveArray = sys::OptixBuildInputCurveArray {
            curveType: self.kind.raw(),
            numPrimitives: segments,
            vertexBuffers: vertex_buffers,
            numVertices: vertices,
            vertexStrideInBytes: stride::<[f32; 3]>(),
            widthBuffers: width_buffers,
            widthStrideInBytes: stride::<f32>(),
            normalBuffers: normal_buffers,
            normalStrideInBytes: if has_normals { stride::<[f32; 3]>() } else { 0 },
            indexBuffer: self.segments.cu_deviceptr(),
            indexStrideInBytes: stride::<u32>(),
            flag: self.flags.0,
            primitiveIndexOffset: self.primitive_offset,
            endcapFlags: if self.endcaps {
                sys::OptixCurveEndcapFlags_OPTIX_CURVE_ENDCAP_ON
            } else {
                sys::OptixCurveEndcapFlags_OPTIX_CURVE_ENDCAP_DEFAULT
            } as u32,
        };
        out.inputs.push(input);
        out.shape.push(Item::Curve {
            kind: self.kind.raw(),
            segments,
            vertices,
            normals: has_normals,
        });
        Ok(())
    }
}

impl Gas for Spheres<'_> {
    fn push(&self, cx: &Cx<'_>, out: &mut Inputs) -> Result<()> {
        let (cptrs, spheres) = self.centers.split(cx.keys)?;
        let (single, (rptrs, radii)) = match self.radii {
            Radii::Single(k) => (true, k.split(cx.keys)?),
            Radii::PerSphere(k) => (false, k.split(cx.keys)?),
        };
        if single && radii != 1 {
            return Err(Error::Invalid(
                "a single-radius buffer holds exactly one radius per key",
            ));
        }
        if !single && radii != spheres {
            return Err(Error::Invalid(
                "per-sphere radii need one radius per center",
            ));
        }
        if spheres == 0 {
            return Err(Error::Invalid("a sphere input needs at least one sphere"));
        }
        prim_range(spheres, self.primitive_offset)?;
        let (flags, records, sbt_ptr, sbt_size, sbt_stride) =
            out.sbt(self.flags, self.sbt_offsets, spheres)?;
        let vertex_buffers = out.keep_ptrs(cptrs);
        let radius_buffers = out.keep_ptrs(rptrs);
        let mut input = sys::OptixBuildInput {
            type_: sys::OptixBuildInputType_OPTIX_BUILD_INPUT_TYPE_SPHERES,
            ..Default::default()
        };
        input.__bindgen_anon_1.sphereArray = sys::OptixBuildInputSphereArray {
            vertexBuffers: vertex_buffers,
            vertexStrideInBytes: stride::<[f32; 3]>(),
            numVertices: spheres,
            radiusBuffers: radius_buffers,
            radiusStrideInBytes: stride::<f32>(),
            singleRadius: i32::from(single),
            flags,
            numSbtRecords: records,
            sbtIndexOffsetBuffer: sbt_ptr,
            sbtIndexOffsetSizeInBytes: sbt_size,
            sbtIndexOffsetStrideInBytes: sbt_stride,
            primitiveIndexOffset: self.primitive_offset,
        };
        out.inputs.push(input);
        out.shape.push(Item::Sphere {
            spheres,
            single,
            sbt: records,
        });
        Ok(())
    }
}

impl Gas for CustomPrimitives<'_> {
    fn push(&self, cx: &Cx<'_>, out: &mut Inputs) -> Result<()> {
        let (aptrs, prims) = self.aabbs.split(cx.keys)?;
        if aptrs
            .iter()
            .any(|p| p % u64::from(sys::OPTIX_AABB_BUFFER_BYTE_ALIGNMENT) != 0)
        {
            return Err(Error::Invalid(
                "AABB buffer is below OptiX's AABB alignment",
            ));
        }
        if prims == 0 {
            return Err(Error::Invalid(
                "a custom-primitive input needs at least one box",
            ));
        }
        prim_range(prims, self.primitive_offset)?;
        let (flags, records, sbt_ptr, sbt_size, sbt_stride) =
            out.sbt(self.flags, self.sbt_offsets, prims)?;
        let aabb_buffers = out.keep_ptrs(aptrs);
        let mut input = sys::OptixBuildInput {
            type_: sys::OptixBuildInputType_OPTIX_BUILD_INPUT_TYPE_CUSTOM_PRIMITIVES,
            ..Default::default()
        };
        input.__bindgen_anon_1.customPrimitiveArray = sys::OptixBuildInputCustomPrimitiveArray {
            aabbBuffers: aabb_buffers,
            numPrimitives: prims,
            strideInBytes: stride::<Aabb>(),
            flags,
            numSbtRecords: records,
            sbtIndexOffsetBuffer: sbt_ptr,
            sbtIndexOffsetSizeInBytes: sbt_size,
            sbtIndexOffsetStrideInBytes: sbt_stride,
            primitiveIndexOffset: self.primitive_offset,
        };
        out.inputs.push(input);
        out.shape.push(Item::Custom {
            prims,
            sbt: records,
        });
        Ok(())
    }
}

fn gas_inputs<G: Gas>(items: &[G], cx: &Cx<'_>) -> Result<Inputs> {
    if items.is_empty() {
        return Err(Error::Invalid(
            "a geometry structure needs at least one build input",
        ));
    }
    let mut out = Inputs::default();
    for item in items {
        item.push(cx, &mut out)?;
    }
    Ok(out)
}

/// One instance-array input over `raw` instances of `kind`'s device array `device`.
fn instance_input(
    kind: sys::OptixBuildInputType,
    device: DeviceBuffer<u8>,
    n: u32,
    item: Item,
    children: Vec<Child>,
) -> Inputs {
    let mut input = sys::OptixBuildInput {
        type_: kind,
        ..Default::default()
    };
    input.__bindgen_anon_1.instanceArray = sys::OptixBuildInputInstanceArray {
        instances: device.cu_deviceptr(),
        numInstances: n,
        instanceStride: 0,
    };
    Inputs {
        inputs: vec![input],
        shape: vec![item],
        _device: vec![device],
        refs: Refs {
            children,
            micromaps: Vec::new(),
        },
        ..Default::default()
    }
}

fn instance_inputs(cx: &Cx<'_>, instances: &[Instance]) -> Result<Inputs> {
    if instances.is_empty() {
        return Err(Error::Invalid(
            "an instance structure needs at least one instance",
        ));
    }
    check_children(cx.ctx, instances)?;
    let raw: Vec<sys::OptixInstance> = instances.iter().map(Instance::raw).collect();
    let n = count(raw.len(), "too many instances")?;
    // SAFETY: `OptixInstance` has no padding (asserted above).
    let device = unsafe { upload(cx.stream, &raw) }?;
    if device.cu_deviceptr() % u64::from(sys::OPTIX_INSTANCE_BYTE_ALIGNMENT) != 0 {
        return Err(Error::Invalid(
            "allocator returned instances below OptiX's instance alignment",
        ));
    }
    let children = instances.iter().map(|i| i.child.clone()).collect();
    Ok(instance_input(
        sys::OptixBuildInputType_OPTIX_BUILD_INPUT_TYPE_INSTANCES,
        device,
        n,
        Item::Instances(n),
        children,
    ))
}

fn pointer_inputs(cx: &Cx<'_>, refs: &[(&DeviceInstances, u32)]) -> Result<Inputs> {
    if refs.is_empty() {
        return Err(Error::Invalid(
            "an instance structure needs at least one instance",
        ));
    }
    let mut ptrs = Vec::with_capacity(refs.len());
    let mut children = Vec::with_capacity(refs.len());
    for (set, index) in refs {
        if !Arc::ptr_eq(&set.stream, cx.stream) {
            return Err(Error::Invalid(
                "device instances were uploaded on a different stream than the build",
            ));
        }
        if !Arc::ptr_eq(&set.ctx, cx.ctx) {
            return Err(Error::Invalid(
                "device instances from another device context",
            ));
        }
        let child = set.children.get(*index as usize).ok_or(Error::Invalid(
            "instance index out of range of its device instances",
        ))?;
        // Each element stays on the instance alignment: the base is aligned and the size is a
        // multiple of it (asserted above).
        ptrs.push(
            set.buffer.cu_deviceptr() + u64::from(*index) * size_of::<sys::OptixInstance>() as u64,
        );
        children.push(child.clone());
    }
    let n = count(ptrs.len(), "too many instances")?;
    // SAFETY: `u64` has no padding.
    let device = unsafe { upload(cx.stream, &ptrs) }?;
    Ok(instance_input(
        sys::OptixBuildInputType_OPTIX_BUILD_INPUT_TYPE_INSTANCE_POINTERS,
        device,
        n,
        Item::InstancePointers(n),
        children,
    ))
}

/// A property [`Accel::emit`] writes.
pub enum Emit<'a> {
    /// The compacted size in bytes (one `u64`); needs [`BuildOptions::compact`].
    CompactedSize(&'a mut DeviceBuffer<u64>),
    /// The bounds, one box per motion key (one without motion).
    Aabbs(&'a mut DeviceBuffer<Aabb>),
}

/// Opaque data describing a built structure or micromap array, for relocation.
#[derive(Clone, Copy, Debug)]
pub struct RelocationInfo(pub(crate) sys::OptixRelocationInfo);

impl RelocationInfo {
    /// Whether data described by `self` can be relocated into `ctx`
    /// (`optixCheckRelocationCompatibility`); a device is always compatible with itself.
    pub fn compatible(&self, ctx: &DeviceContext) -> Result<bool> {
        let mut ok = 0;
        ctx.cuda().bind_to_thread()?;
        // SAFETY: `self.0` came from OptiX; `ok` is writable.
        check("optixCheckRelocationCompatibility", unsafe {
            entry!(ctx.table(), optixCheckRelocationCompatibility)(ctx.raw(), &self.0, &mut ok)
        })?;
        Ok(ok != 0)
    }
}

/// Which references a relocated structure uses ([`Accel::relocate`]).
#[derive(Clone, Copy)]
pub enum Relink<'a> {
    /// Keep the source's children and micromap arrays (same device context only).
    Keep,
    /// Instance structures: the (relocated) structure per instance, in build order.
    Children(&'a [Child]),
    /// Triangle structures: the (relocated) micromap array per mesh, given exactly for the meshes
    /// built with one.
    Micromaps(&'a [Option<Arc<OpacityMicromapArray>>]),
}

/// A built acceleration structure.
pub struct Accel {
    handle: AtomicU64,
    buffer: DeviceBuffer<u8>,
    flags: u32,
    motion: sys::OptixMotionOptions,
    /// Buffers per per-key input (motion keys, or 1).
    keys: usize,
    shape: Vec<Item>,
    refs: Mutex<Refs>,
    ctx: Arc<DeviceContext>,
}

impl Accel {
    /// A geometry structure over triangle `meshes`, built on `stream` with temp memory from
    /// `scratch`.
    pub fn triangles(
        ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        meshes: &[TriangleMesh<'_>],
        opts: BuildOptions,
    ) -> Result<Arc<Self>> {
        Self::geometry(ctx, stream, scratch, meshes, opts)
    }

    /// A geometry structure over curve inputs (the pipeline needs the matching built-in
    /// intersection module and [`CurveType::flag`] in its primitive types).
    pub fn curves(
        ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        curves: &[Curves<'_>],
        opts: BuildOptions,
    ) -> Result<Arc<Self>> {
        Self::geometry(ctx, stream, scratch, curves, opts)
    }

    /// A geometry structure over sphere inputs.
    pub fn spheres(
        ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        spheres: &[Spheres<'_>],
        opts: BuildOptions,
    ) -> Result<Arc<Self>> {
        Self::geometry(ctx, stream, scratch, spheres, opts)
    }

    /// A geometry structure over custom primitives (AABBs).
    pub fn custom(
        ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        prims: &[CustomPrimitives<'_>],
        opts: BuildOptions,
    ) -> Result<Arc<Self>> {
        Self::geometry(ctx, stream, scratch, prims, opts)
    }

    /// An instance structure over `instances`, built on `stream` with temp memory from `scratch`.
    pub fn instances(
        ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        instances: &[Instance],
        opts: BuildOptions,
    ) -> Result<Arc<Self>> {
        let cx = Self::cx(ctx, stream, &opts)?;
        let inputs = instance_inputs(&cx, instances)?;
        Self::build(ctx, stream, scratch, inputs, &opts)
    }

    /// An instance structure over pointers to device-resident instances: `(set, index)` per
    /// instance. The sets must have been uploaded on `stream`.
    pub fn instance_pointers(
        ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        instances: &[(&DeviceInstances, u32)],
        opts: BuildOptions,
    ) -> Result<Arc<Self>> {
        let cx = Self::cx(ctx, stream, &opts)?;
        let inputs = pointer_inputs(&cx, instances)?;
        Self::build(ctx, stream, scratch, inputs, &opts)
    }

    /// Builds or updates an instance acceleration structure directly into `target`, reading from
    /// a persistent [`InstanceBuffer`] with zero steady-state VRAM allocations or frees.
    ///
    /// - When `target` is `Some(existing)`, `!force_rebuild`, `existing` was built with
    ///   [`BuildOptions::allow_update`], and `d_instances.len()` matches the built instance count,
    ///   executes `OPTIX_BUILD_OPERATION_UPDATE` in place and returns `Ok(true)`.
    /// - Otherwise executes `OPTIX_BUILD_OPERATION_BUILD` and returns `Ok(false)`. When `target`
    ///   is uniquely owned (`Arc::strong_count == 1`), has matching build flags/motion, and its
    ///   device buffer already has capacity `>= sizes.outputSizeInBytes`, the build reuses
    ///   `existing`'s device buffer in place with zero `cuMemAlloc`/`cuMemFree`.
    ///
    /// # Safety
    ///
    /// No launch on another stream may read `target` while this build or update runs: order all
    /// launches and updates on `stream` or synchronize before calling.
    pub unsafe fn instances_into(
        ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        d_instances: &InstanceBuffer,
        opts: BuildOptions,
        force_rebuild: bool,
        target: &mut Option<Arc<Self>>,
    ) -> Result<bool> {
        same_ctx(stream, ctx)?;
        if !Arc::ptr_eq(&d_instances.ctx, ctx) {
            return Err(Error::Invalid(
                "instance buffer belongs to another OptiX device context",
            ));
        }
        if !Arc::ptr_eq(&d_instances.stream, stream) {
            return Err(Error::Invalid(
                "instance buffer was uploaded on a different stream than the build",
            ));
        }
        if d_instances.is_empty() {
            return Err(Error::Invalid(
                "an instance structure needs at least one instance",
            ));
        }
        if opts.compact {
            return Err(Error::Invalid(
                "Accel::instances_into does not support compaction (use persistent uncompacted IAS)",
            ));
        }
        let n = count(d_instances.len(), "too many instances")?;
        let flags = opts.flags();
        let motion = opts.motion_sys()?;
        let keys = usize::from(opts.motion_keys);

        let mut build_input = sys::OptixBuildInput {
            type_: sys::OptixBuildInputType_OPTIX_BUILD_INPUT_TYPE_INSTANCES,
            ..Default::default()
        };
        build_input.__bindgen_anon_1.instanceArray = sys::OptixBuildInputInstanceArray {
            instances: d_instances.device_ptr(),
            numInstances: n,
            instanceStride: 0,
        };
        let inputs_slice = [build_input];
        let want_shape = Item::Instances(n);

        // Fast path: in-place `OPTIX_BUILD_OPERATION_UPDATE` when count & topology are unchanged.
        if !force_rebuild
            && let Some(existing) = target.as_ref()
            && (existing.flags & sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_UPDATE as u32) != 0
            && existing.shape.as_slice() == [want_shape.clone()]
        {
            let accel_opts = sys::OptixAccelBuildOptions {
                buildFlags: existing.flags,
                operation: sys::OptixBuildOperation_OPTIX_BUILD_OPERATION_UPDATE,
                motionOptions: existing.motion,
            };
            let sizes = Self::sizes(ctx, &accel_opts, &inputs_slice)?;
            let (temp, temp_len) = scratch.reserve(stream, sizes.tempUpdateSizeInBytes)?;
            let mut handle = existing.handle();
            // SAFETY: `d_instances` holds `n` aligned `OptixInstance`s on `stream`; `existing.buffer`
            // holds the previously built IAS; caller orders all readers on `stream`.
            check("optixAccelBuild(instances_into update)", unsafe {
                entry!(ctx.table(), optixAccelBuild)(
                    ctx.raw(),
                    stream.cu_stream(),
                    &accel_opts,
                    inputs_slice.as_ptr(),
                    1,
                    temp,
                    temp_len,
                    existing.buffer.cu_deviceptr(),
                    existing.buffer.len(),
                    &mut handle,
                    std::ptr::null(),
                    0,
                )
            })?;
            existing.handle.store(handle, Ordering::Release);
            let mut refs = existing
                .refs
                .lock()
                .map_err(|_| Error::Invalid("reference list poisoned by a panic"))?;
            refs.children.clear();
            refs.children.extend_from_slice(&d_instances.children);
            return Ok(true);
        }

        // Full BUILD path: reuse `target`'s device buffer in place whenever uniquely owned & large enough.
        let accel_opts = sys::OptixAccelBuildOptions {
            buildFlags: flags,
            operation: sys::OptixBuildOperation_OPTIX_BUILD_OPERATION_BUILD,
            motionOptions: motion,
        };
        let sizes = Self::sizes(ctx, &accel_opts, &inputs_slice)?;
        let (temp, temp_len) = scratch.reserve(stream, sizes.tempSizeInBytes)?;

        let reused_cap = target.as_ref().map_or(0, |a| a.buffer.len());
        let can_reuse_in_place = target.as_mut().is_some_and(|arc| {
            Arc::get_mut(arc).is_some() && arc.buffer.len() >= sizes.outputSizeInBytes
        });

        if can_reuse_in_place && let Some(arc) = target.as_mut()
            && let Some(existing) = Arc::get_mut(arc)
        {
            let mut handle = 0;
            // SAFETY: `existing` is uniquely owned, its buffer is `OPTIX_ACCEL_BUFFER_BYTE_ALIGNMENT`
            // aligned with length `>= sizes.outputSizeInBytes`, and all readers are stream-ordered.
            check("optixAccelBuild(instances_into reuse)", unsafe {
                entry!(ctx.table(), optixAccelBuild)(
                    ctx.raw(),
                    stream.cu_stream(),
                    &accel_opts,
                    inputs_slice.as_ptr(),
                    1,
                    temp,
                    temp_len,
                    existing.buffer.cu_deviceptr(),
                    existing.buffer.len(),
                    &mut handle,
                    std::ptr::null(),
                    0,
                )
            })?;
            existing.handle.store(handle, Ordering::Release);
            existing.flags = flags;
            existing.motion = motion;
            existing.keys = keys;
            existing.shape.clear();
            existing.shape.push(want_shape);
            let mut refs = existing
                .refs
                .lock()
                .map_err(|_| Error::Invalid("reference list poisoned by a panic"))?;
            refs.children.clear();
            refs.children.extend_from_slice(&d_instances.children);
            refs.micromaps.clear();
            return Ok(false);
        }

        // Grow-only allocation (2x geometric growth) to avoid future reallocations.
        let alloc_bytes = sizes
            .outputSizeInBytes
            .max(reused_cap.saturating_mul(2));
        within(
            opts.budget,
            scratch
                .growth(sizes.tempSizeInBytes)
                .saturating_add(alloc_bytes),
        )?;
        // SAFETY: OptiX writes the output buffer before any launch reads it on `stream`.
        let output = unsafe { alloc(stream, alloc_bytes, sys::OPTIX_ACCEL_BUFFER_BYTE_ALIGNMENT) }?;
        let mut handle = 0;
        // SAFETY: `output` has `alloc_bytes >= sizes.outputSizeInBytes` aligned bytes on `stream`.
        check("optixAccelBuild(instances_into alloc)", unsafe {
            entry!(ctx.table(), optixAccelBuild)(
                ctx.raw(),
                stream.cu_stream(),
                &accel_opts,
                inputs_slice.as_ptr(),
                1,
                temp,
                temp_len,
                output.cu_deviceptr(),
                output.len(),
                &mut handle,
                std::ptr::null(),
                0,
            )
        })?;
        *target = Some(Arc::new(Self {
            handle: AtomicU64::new(handle),
            buffer: output,
            flags,
            motion,
            keys,
            shape: vec![want_shape],
            refs: Mutex::new(Refs {
                children: d_instances.children.clone(),
                micromaps: Vec::new(),
            }),
            ctx: ctx.clone(),
        }));
        Ok(false)
    }

    fn cx<'a>(
        ctx: &'a Arc<DeviceContext>,
        stream: &'a Arc<Stream>,
        opts: &BuildOptions,
    ) -> Result<Cx<'a>> {
        same_ctx(stream, ctx)?;
        opts.motion_sys()?;
        let keys = usize::from(opts.motion_keys);
        Ok(Cx {
            ctx,
            stream,
            keys,
            omm_update: opts.allow_micromap_update,
        })
    }

    fn geometry<G: Gas>(
        ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        items: &[G],
        opts: BuildOptions,
    ) -> Result<Arc<Self>> {
        let cx = Self::cx(ctx, stream, &opts)?;
        let inputs = gas_inputs(items, &cx)?;
        Self::build(ctx, stream, scratch, inputs, &opts)
    }

    fn sizes(
        ctx: &DeviceContext,
        accel_opts: &sys::OptixAccelBuildOptions,
        inputs: &[sys::OptixBuildInput],
    ) -> Result<sys::OptixAccelBufferSizes> {
        let n = count(inputs.len(), "too many build inputs")?;
        let mut sizes = sys::OptixAccelBufferSizes::default();
        ctx.cuda().bind_to_thread()?;
        // SAFETY: `inputs` and the host arrays and buffers they point at are alive for the call.
        check("optixAccelComputeMemoryUsage", unsafe {
            entry!(ctx.table(), optixAccelComputeMemoryUsage)(
                ctx.raw(),
                accel_opts,
                inputs.as_ptr(),
                n,
                &mut sizes,
            )
        })?;
        Ok(sizes)
    }

    fn build(
        ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        inputs: Inputs,
        opts: &BuildOptions,
    ) -> Result<Arc<Self>> {
        let t = ctx.table();
        let flags = opts.flags();
        let motion = opts.motion_sys()?;
        let keys = usize::from(opts.motion_keys);
        let accel_opts = sys::OptixAccelBuildOptions {
            buildFlags: flags,
            operation: sys::OptixBuildOperation_OPTIX_BUILD_OPERATION_BUILD,
            motionOptions: motion,
        };
        let sizes = Self::sizes(ctx, &accel_opts, &inputs.inputs)?;
        within(
            opts.budget,
            scratch
                .growth(sizes.tempSizeInBytes)
                .saturating_add(sizes.outputSizeInBytes),
        )?;
        let (temp, temp_len) = scratch.reserve(stream, sizes.tempSizeInBytes)?;
        // SAFETY: OptiX writes the output before anything reads it.
        let output = unsafe {
            alloc(
                stream,
                sizes.outputSizeInBytes,
                sys::OPTIX_ACCEL_BUFFER_BYTE_ALIGNMENT,
            )
        }?;
        let compacted_size = if opts.compact {
            Some(DeviceBuffer::<u64>::zeroed(stream, 1)?)
        } else {
            None
        };
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
        let (shape, refs) = (inputs.shape, inputs.refs);
        let make = |handle, buffer| {
            Arc::new(Self {
                handle: AtomicU64::new(handle),
                buffer,
                flags,
                motion,
                keys,
                shape,
                refs: Mutex::new(refs),
                ctx: ctx.clone(),
            })
        };

        let Some(compacted_size) = compacted_size else {
            return Ok(make(handle, output));
        };
        let words = compacted_size.to_host_vec(stream)?;
        let word = words
            .first()
            .copied()
            .ok_or(Error::Invalid("compacted size readback returned no value"))?;
        let size =
            usize::try_from(word).map_err(|_| Error::Invalid("compacted size overflows usize"))?;
        if size == 0 || size >= sizes.outputSizeInBytes {
            return Ok(make(handle, output));
        }
        // SAFETY: written by the compaction below before any read.
        let compact = unsafe { alloc(stream, size, sys::OPTIX_ACCEL_BUFFER_BYTE_ALIGNMENT) }?;
        let mut compact_handle = 0;
        // SAFETY: `handle` was built with ALLOW_COMPACTION; `compact` holds `size` bytes on
        // `stream`; the source buffer's free is ordered after this call.
        check("optixAccelCompact", unsafe {
            entry!(t, optixAccelCompact)(
                ctx.raw(),
                stream.cu_stream(),
                handle,
                compact.cu_deviceptr(),
                size,
                &mut compact_handle,
            )
        })?;
        drop(output);
        Ok(make(compact_handle, compact))
    }

    /// The shared context of every refit: the build's motion keys and micromap-update flag.
    fn refit_cx<'a>(&'a self, stream: &'a Arc<Stream>) -> Result<Cx<'a>> {
        same_ctx(stream, &self.ctx)?;
        Ok(Cx {
            ctx: &self.ctx,
            stream,
            keys: self.keys,
            omm_update: self.flags
                & sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_OPACITY_MICROMAP_UPDATE as u32
                != 0,
        })
    }

    /// # Safety
    ///
    /// See [`Accel::refit_triangles`].
    unsafe fn refit<G: Gas>(
        &self,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        items: &[G],
    ) -> Result<()> {
        let inputs = gas_inputs(items, &self.refit_cx(stream)?)?;
        // SAFETY: forwarded contract.
        unsafe { self.update(stream, scratch, inputs) }
    }

    /// Refits a triangle structure to moved vertices: same meshes, counts, formats, SBT records
    /// and motion keys.
    ///
    /// # Safety
    ///
    /// No launch may read this structure (directly or through an instance structure) while the
    /// refit runs: order such launches on `stream` or synchronize. Instance structures over this
    /// one keep stale bounds until they are refitted or rebuilt too.
    pub unsafe fn refit_triangles(
        &self,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        meshes: &[TriangleMesh<'_>],
    ) -> Result<()> {
        // SAFETY: forwarded contract.
        unsafe { self.refit(stream, scratch, meshes) }
    }

    /// Refits a curve structure to moved control points or widths.
    ///
    /// # Safety
    ///
    /// As [`Accel::refit_triangles`].
    pub unsafe fn refit_curves(
        &self,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        curves: &[Curves<'_>],
    ) -> Result<()> {
        // SAFETY: forwarded contract.
        unsafe { self.refit(stream, scratch, curves) }
    }

    /// Refits a sphere structure to moved centers or radii.
    ///
    /// # Safety
    ///
    /// As [`Accel::refit_triangles`].
    pub unsafe fn refit_spheres(
        &self,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        spheres: &[Spheres<'_>],
    ) -> Result<()> {
        // SAFETY: forwarded contract.
        unsafe { self.refit(stream, scratch, spheres) }
    }

    /// Refits a custom-primitive structure to new boxes.
    ///
    /// # Safety
    ///
    /// As [`Accel::refit_triangles`].
    pub unsafe fn refit_custom(
        &self,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        prims: &[CustomPrimitives<'_>],
    ) -> Result<()> {
        // SAFETY: forwarded contract.
        unsafe { self.refit(stream, scratch, prims) }
    }

    /// Refits an instance structure to new transforms, masks, SBT offsets or instanced
    /// structures; the instance count must stay the same.
    ///
    /// # Safety
    ///
    /// As [`Accel::refit_triangles`].
    pub unsafe fn refit_instances(
        &self,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        instances: &[Instance],
    ) -> Result<()> {
        let inputs = instance_inputs(&self.refit_cx(stream)?, instances)?;
        // SAFETY: forwarded contract.
        unsafe { self.update(stream, scratch, inputs) }
    }

    /// Refits an instance-pointer structure; the instance count must stay the same.
    ///
    /// # Safety
    ///
    /// As [`Accel::refit_triangles`].
    pub unsafe fn refit_instance_pointers(
        &self,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        instances: &[(&DeviceInstances, u32)],
    ) -> Result<()> {
        let inputs = pointer_inputs(&self.refit_cx(stream)?, instances)?;
        // SAFETY: forwarded contract.
        unsafe { self.update(stream, scratch, inputs) }
    }

    /// # Safety
    ///
    /// See [`Accel::refit_triangles`].
    unsafe fn update(
        &self,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        inputs: Inputs,
    ) -> Result<()> {
        if self.flags & sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_UPDATE as u32 == 0 {
            return Err(Error::Invalid(
                "structure was not built with BuildOptions::allow_update",
            ));
        }
        if inputs.shape != self.shape {
            return Err(Error::Invalid(
                "refit inputs differ in shape from the build (kind, counts, formats or micromaps)",
            ));
        }
        let accel_opts = sys::OptixAccelBuildOptions {
            buildFlags: self.flags,
            operation: sys::OptixBuildOperation_OPTIX_BUILD_OPERATION_UPDATE,
            motionOptions: self.motion,
        };
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
        let mut refs = self
            .refs
            .lock()
            .map_err(|_| Error::Invalid("reference list poisoned by a panic"))?;
        *refs = inputs.refs;
        Ok(())
    }

    /// Writes a post-build property on `stream` (`optixAccelEmitProperty`).
    ///
    /// # Safety
    ///
    /// The structure and the output buffer must stay alive, and the structure unrefitted, until
    /// the write completes on `stream`; read the output on `stream` or after synchronizing it.
    pub unsafe fn emit(&self, stream: &Stream, out: Emit<'_>) -> Result<()> {
        same_ctx(stream, &self.ctx)?;
        let (result, type_) = match out {
            Emit::CompactedSize(_)
                if self.flags & sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_COMPACTION as u32
                    == 0 =>
            {
                return Err(Error::Invalid(
                    "compacted size needs a structure built with BuildOptions::compact",
                ));
            }
            Emit::CompactedSize(b) if b.len() != 1 => {
                return Err(Error::Invalid(
                    "compacted size output holds exactly one u64",
                ));
            }
            Emit::CompactedSize(b) => (
                b.cu_deviceptr(),
                sys::OptixAccelPropertyType_OPTIX_PROPERTY_TYPE_COMPACTED_SIZE,
            ),
            Emit::Aabbs(b) if b.len() != self.keys => {
                return Err(Error::Invalid("AABB output needs one box per motion key"));
            }
            Emit::Aabbs(b) => (
                b.cu_deviceptr(),
                sys::OptixAccelPropertyType_OPTIX_PROPERTY_TYPE_AABBS,
            ),
        };
        let desc = sys::OptixAccelEmitDesc { result, type_ };
        self.ctx.cuda().bind_to_thread()?;
        // SAFETY: owned handle; the output holds the property's size; lifetimes per the contract.
        check("optixAccelEmitProperty", unsafe {
            entry!(self.ctx.table(), optixAccelEmitProperty)(
                self.ctx.raw(),
                stream.cu_stream(),
                self.handle(),
                &desc,
            )
        })
    }

    /// Relocation data for this structure (`optixAccelGetRelocationInfo`).
    pub fn relocation_info(&self) -> Result<RelocationInfo> {
        let mut info = sys::OptixRelocationInfo::default();
        self.ctx.cuda().bind_to_thread()?;
        // SAFETY: owned handle of this context; `info` is writable.
        check("optixAccelGetRelocationInfo", unsafe {
            entry!(self.ctx.table(), optixAccelGetRelocationInfo)(
                self.ctx.raw(),
                self.handle(),
                &mut info,
            )
        })?;
        Ok(RelocationInfo(info))
    }

    /// A copy of this structure in a new buffer on `ctx` (possibly another device), patched by
    /// `optixAccelRelocate`. Order: relocation info, compatibility check against `ctx`, budget
    /// check, allocation and copy on `stream`, relocation on `stream`. `relink` names the
    /// references of the copy; [`Relink::Keep`] is only valid on the same context or for
    /// structures without children and micromaps. The copy keeps the source's build flags, so
    /// it can be refitted or compacted like the source.
    ///
    /// # Safety
    ///
    /// The source's build and refits must be complete or ordered before `stream`, and no refit of
    /// the source may run until the copy completes. Relinked children and micromap arrays must
    /// match the source's in count and order of use (instances, meshes).
    pub unsafe fn relocate(
        &self,
        ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        relink: Relink<'_>,
        budget: Option<usize>,
    ) -> Result<Arc<Self>> {
        same_ctx(stream, ctx)?;
        let info = self.relocation_info()?;
        if !info.compatible(ctx)? {
            return Err(Error::Invalid(
                "structure is not relocatable to this device context",
            ));
        }
        let extra = match relink {
            Relink::Children(c) => c
                .len()
                .saturating_mul(size_of::<sys::OptixTraversableHandle>()),
            _ => 0,
        };
        within(budget, self.buffer.len().saturating_add(extra))?;
        let mut shape = self.shape.clone();
        let mut handles = None;
        let mut inputs: Vec<sys::OptixRelocateInput> = Vec::new();
        let refs = match relink {
            Relink::Keep => {
                let refs = self
                    .refs
                    .lock()
                    .map_err(|_| Error::Invalid("reference list poisoned by a panic"))?
                    .clone();
                if !Arc::ptr_eq(ctx, &self.ctx)
                    && (!refs.children.is_empty() || !refs.micromaps.is_empty())
                {
                    return Err(Error::Invalid(
                        "relocating to another device context needs Relink::Children or Relink::Micromaps",
                    ));
                }
                refs
            }
            Relink::Children(children) => {
                let (kind, n) = match self.shape.as_slice() {
                    [Item::Instances(n)] => (
                        sys::OptixBuildInputType_OPTIX_BUILD_INPUT_TYPE_INSTANCES,
                        *n,
                    ),
                    [Item::InstancePointers(n)] => (
                        sys::OptixBuildInputType_OPTIX_BUILD_INPUT_TYPE_INSTANCE_POINTERS,
                        *n,
                    ),
                    _ => {
                        return Err(Error::Invalid(
                            "Relink::Children applies to instance structures",
                        ));
                    }
                };
                if children.len() != n as usize {
                    return Err(Error::Invalid(
                        "Relink::Children needs one structure per instance",
                    ));
                }
                if children.iter().any(|c| !Arc::ptr_eq(c.context(), ctx)) {
                    return Err(Error::Invalid(
                        "relinked child from another device context than the target",
                    ));
                }
                let raw: Vec<sys::OptixTraversableHandle> =
                    children.iter().map(|c| c.handle()).collect();
                // SAFETY: `u64` handles have no padding.
                let device = unsafe { upload(stream, &raw) }?;
                let mut input = sys::OptixRelocateInput {
                    type_: kind,
                    ..Default::default()
                };
                input.__bindgen_anon_1.instanceArray = sys::OptixRelocateInputInstanceArray {
                    numInstances: n,
                    traversableHandles: device.cu_deviceptr(),
                };
                inputs.push(input);
                handles = Some(device);
                Refs {
                    children: children.to_vec(),
                    micromaps: Vec::new(),
                }
            }
            Relink::Micromaps(arrays) => {
                if arrays.len() != shape.len() {
                    return Err(Error::Invalid("Relink::Micromaps needs one entry per mesh"));
                }
                let mut micromaps = Vec::new();
                for (item, array) in shape.iter_mut().zip(arrays) {
                    let Item::Tri { sbt, omm, .. } = item else {
                        return Err(Error::Invalid(
                            "Relink::Micromaps applies to triangle structures",
                        ));
                    };
                    if omm.is_some() != array.is_some() {
                        return Err(Error::Invalid(
                            "a relinked micromap array must be given exactly for the meshes built with one",
                        ));
                    }
                    let ptr = match array {
                        Some(a) if !Arc::ptr_eq(a.context(), ctx) => {
                            return Err(Error::Invalid(
                                "relinked micromap array from another device context than the target",
                            ));
                        }
                        Some(a) => {
                            micromaps.push(a.clone());
                            a.ptr()
                        }
                        None => 0,
                    };
                    // A fixed (non-updatable) array is part of the refit shape: follow the relink.
                    if let Some(key) = omm
                        && *key != 0
                    {
                        *key = ptr;
                    }
                    let mut input = sys::OptixRelocateInput {
                        type_: sys::OptixBuildInputType_OPTIX_BUILD_INPUT_TYPE_TRIANGLES,
                        ..Default::default()
                    };
                    input.__bindgen_anon_1.triangleArray = sys::OptixRelocateInputTriangleArray {
                        numSbtRecords: *sbt,
                        opacityMicromap: sys::OptixRelocateInputOpacityMicromap {
                            opacityMicromapArray: ptr,
                        },
                    };
                    inputs.push(input);
                }
                Refs {
                    children: Vec::new(),
                    micromaps,
                }
            }
        };
        // SAFETY: filled by the copy below.
        let target = unsafe {
            alloc(
                stream,
                self.buffer.len(),
                sys::OPTIX_ACCEL_BUFFER_BYTE_ALIGNMENT,
            )
        }?;
        // SAFETY: the source's writes are ordered before `stream` (contract); `target` is fresh.
        unsafe { copy_bytes(stream, &target, &self.buffer) }?;
        let mut handle = 0;
        ctx.cuda().bind_to_thread()?;
        // SAFETY: `target` holds a copy of the source (same size, accel alignment); `inputs` and
        // the handle buffer they point at live for the call and, being stream-ordered, until the
        // relocation has read them.
        check("optixAccelRelocate", unsafe {
            entry!(ctx.table(), optixAccelRelocate)(
                ctx.raw(),
                stream.cu_stream(),
                &info.0,
                if inputs.is_empty() {
                    std::ptr::null()
                } else {
                    inputs.as_ptr()
                },
                inputs.len(),
                target.cu_deviceptr(),
                target.len(),
                &mut handle,
            )
        })?;
        drop(handles);
        Ok(Arc::new(Self {
            handle: AtomicU64::new(handle),
            buffer: target,
            flags: self.flags,
            motion: self.motion,
            keys: self.keys,
            shape,
            refs: Mutex::new(refs),
            ctx: ctx.clone(),
        }))
    }

    /// The handle device code traces against (`optixTrace`'s first argument).
    pub fn handle(&self) -> sys::OptixTraversableHandle {
        self.handle.load(Ordering::Acquire)
    }

    /// Bytes the structure occupies on the device.
    pub fn size_bytes(&self) -> usize {
        self.buffer.len()
    }

    /// The device context the structure was built on.
    pub fn context(&self) -> &Arc<DeviceContext> {
        &self.ctx
    }
}
