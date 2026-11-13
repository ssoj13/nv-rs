//! Opacity micromaps: [`OpacityMicromapArray`] (built, owned, relocatable), [`MicromapUse`]
//! (attaching an array to a [`crate::TriangleMesh`]) and the host helpers of `optix_micromap.h`.
//!
//! **What.** A micromap subdivides a triangle into `4^level` micro-triangles, each 1 bit
//! (2-state) or 2 bits (4-state) of opacity, so traversal can skip any-hit calls. An array is
//! built from raw bit data plus one [`MicromapDesc`] per micromap
//! (`optixOpacityMicromapArrayComputeMemoryUsage` / `optixOpacityMicromapArrayBuild`); host
//! descriptors are checked against the data and their histogram is derived, device-written
//! descriptors go through [`OpacityMicromapArray::build_unchecked`]. Arrays relocate like
//! acceleration structures (`optixOpacityMicromapArrayGetRelocationInfo` /
//! `optixOpacityMicromapArrayRelocate`).
//! **Ports.** [`micro_to_base_bary`] and [`base_to_micro_bary`] are `optixMicromapIndexToBaseBarycentrics`
//! and `optixBaseBarycentricsToMicroBarycentrics` from `optix_micromap.h` /
//! `internal/optix_micromap_impl.h`, bit for bit with the header's host build.
//! **Ordering.** Output and descriptor uploads are stream-ordered allocations on the build stream;
//! temp memory comes from a [`Scratch`] of that stream.
//! **Used by** `accel` (triangle inputs keep `Arc`s of their arrays) and `cluster` (cluster
//! arguments reference arrays by [`OpacityMicromapArray::ptr`]).

use crate::accel::{RelocationInfo, Scratch, alloc, copy_bytes, count, same_ctx, upload, within};
use crate::{DeviceContext, Error, Result, check, entry, sys};
use cuda_core::{DeviceBuffer, DeviceCopy, Stream};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Bits per micro-triangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MicromapFormat {
    /// Transparent / opaque (1 bit).
    TwoState,
    /// Transparent / opaque / unknown-transparent / unknown-opaque (2 bits).
    FourState,
}

impl MicromapFormat {
    fn raw(self) -> sys::OptixOpacityMicromapFormat {
        match self {
            Self::TwoState => sys::OptixOpacityMicromapFormat_OPTIX_OPACITY_MICROMAP_FORMAT_2_STATE,
            Self::FourState => sys::OptixOpacityMicromapFormat_OPTIX_OPACITY_MICROMAP_FORMAT_4_STATE,
        }
    }

    fn from_raw(raw: u16) -> Option<Self> {
        [Self::TwoState, Self::FourState].into_iter().find(|f| f.raw() == i32::from(raw))
    }

    fn bits(self) -> u64 {
        match self {
            Self::TwoState => 1,
            Self::FourState => 2,
        }
    }
}

/// One micromap of an array build, layout-identical to `OptixOpacityMicromapDesc`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MicromapDesc {
    /// Byte offset of the micromap's bits in the build's data buffer.
    pub byte_offset: u32,
    /// Subdivision level: `4^level` micro-triangles, level in `0..=12`.
    pub level: u16,
    /// Raw `OptixOpacityMicromapFormat`; use [`MicromapDesc::new`].
    pub format: u16,
}

// SAFETY: `repr(C)` integers (u32, u16, u16): no padding, every bit pattern valid, no ownership.
unsafe impl DeviceCopy for MicromapDesc {}

const _: () = assert!(size_of::<MicromapDesc>() == size_of::<sys::OptixOpacityMicromapDesc>());
const _: () = assert!(std::mem::offset_of!(MicromapDesc, level) == std::mem::offset_of!(sys::OptixOpacityMicromapDesc, subdivisionLevel));
const _: () = assert!(std::mem::offset_of!(MicromapDesc, format) == std::mem::offset_of!(sys::OptixOpacityMicromapDesc, format));
// The tight descriptor stride must be a multiple of the descriptor alignment.
const _: () = assert!(size_of::<MicromapDesc>().is_multiple_of(sys::OPTIX_OPACITY_MICROMAP_DESC_BUFFER_BYTE_ALIGNMENT as usize));

impl MicromapDesc {
    /// A micromap of `format` at `level` whose bits start at `byte_offset`.
    pub fn new(byte_offset: u32, level: u16, format: MicromapFormat) -> Self {
        // Format values are 1 and 2, so the narrowing is exact.
        Self { byte_offset, level, format: format.raw() as u16 }
    }

    /// Bytes of bit data the micromap occupies.
    fn bytes(&self) -> Result<u64> {
        let format = MicromapFormat::from_raw(self.format).ok_or(Error::Invalid("micromap descriptor has an unknown format"))?;
        let level = check_level(u32::from(self.level))?;
        Ok(((1u64 << (2 * level)) * format.bits()).div_ceil(8))
    }
}

fn check_level(level: u32) -> Result<u32> {
    if level > sys::OPTIX_OPACITY_MICROMAP_MAX_SUBDIVISION_LEVEL {
        return Err(Error::Invalid("micromap subdivision level above OPTIX_OPACITY_MICROMAP_MAX_SUBDIVISION_LEVEL"));
    }
    Ok(level)
}

/// How many micromaps of one format and level: a histogram entry of an array build, or a usage
/// count of a triangle input (`OptixOpacityMicromapHistogramEntry` /
/// `OptixOpacityMicromapUsageCount`, which share this layout of meaning).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MicromapCount {
    /// Number of micromaps.
    pub count: u32,
    /// Subdivision level (`0..=12`).
    pub level: u32,
    /// Format.
    pub format: MicromapFormat,
}

impl MicromapCount {
    fn histogram(&self) -> Result<sys::OptixOpacityMicromapHistogramEntry> {
        Ok(sys::OptixOpacityMicromapHistogramEntry { count: self.count, subdivisionLevel: check_level(self.level)?, format: self.format.raw() })
    }

    fn usage(&self) -> Result<sys::OptixOpacityMicromapUsageCount> {
        Ok(sys::OptixOpacityMicromapUsageCount { count: self.count, subdivisionLevel: check_level(self.level)?, format: self.format.raw() })
    }
}

/// Histogram of `descs` per (level, format), in a stable order.
fn histogram_of(descs: &[MicromapDesc]) -> Result<Vec<MicromapCount>> {
    let mut counts: BTreeMap<(u32, MicromapFormat), u32> = BTreeMap::new();
    for d in descs {
        let format = MicromapFormat::from_raw(d.format).ok_or(Error::Invalid("micromap descriptor has an unknown format"))?;
        let n = counts.entry((u32::from(d.level), format)).or_insert(0);
        *n = n.checked_add(1).ok_or(Error::Invalid("too many micromaps"))?;
    }
    Ok(counts.into_iter().map(|((level, format), count)| MicromapCount { count, level, format }).collect())
}

/// Sizes OptiX reports for an array build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MicromapSizes {
    /// Bytes of the array.
    pub output: usize,
    /// Temp bytes of the build.
    pub temp: usize,
}

/// How an array is built.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MicromapOptions {
    /// Favour trace speed over build speed.
    pub prefer_fast_trace: bool,
    /// New device bytes the build may allocate at peak; `None` admits any size.
    pub budget: Option<usize>,
}

impl Default for MicromapOptions {
    /// Fast trace, no budget.
    fn default() -> Self {
        Self { prefer_fast_trace: true, budget: None }
    }
}

impl MicromapOptions {
    fn flags(&self) -> u32 {
        (if self.prefer_fast_trace {
            sys::OptixOpacityMicromapFlags_OPTIX_OPACITY_MICROMAP_FLAG_PREFER_FAST_TRACE
        } else {
            sys::OptixOpacityMicromapFlags_OPTIX_OPACITY_MICROMAP_FLAG_PREFER_FAST_BUILD
        }) as u32
    }
}

/// A built opacity micromap array, owning its device buffer.
pub struct OpacityMicromapArray {
    buffer: DeviceBuffer<u8>,
    len: u32,
    ctx: Arc<DeviceContext>,
}

impl OpacityMicromapArray {
    /// Sizes of a build over micromaps counted by `histogram`
    /// (`optixOpacityMicromapArrayComputeMemoryUsage`).
    pub fn sizes(ctx: &DeviceContext, histogram: &[MicromapCount], opts: MicromapOptions) -> Result<MicromapSizes> {
        let entries = histogram.iter().map(MicromapCount::histogram).collect::<Result<Vec<_>>>()?;
        let input = sys::OptixOpacityMicromapArrayBuildInput {
            flags: opts.flags(),
            numMicromapHistogramEntries: count(entries.len(), "too many histogram entries")?,
            micromapHistogramEntries: entries.as_ptr(),
            ..Default::default()
        };
        Self::sizes_raw(ctx, &input)
    }

    fn sizes_raw(ctx: &DeviceContext, input: &sys::OptixOpacityMicromapArrayBuildInput) -> Result<MicromapSizes> {
        let mut sizes = sys::OptixMicromapBufferSizes { outputSizeInBytes: 0, tempSizeInBytes: 0 };
        ctx.cuda().bind_to_thread()?;
        // SAFETY: `input` and its histogram are alive for the call; `sizes` is writable.
        check("optixOpacityMicromapArrayComputeMemoryUsage", unsafe {
            entry!(ctx.table(), optixOpacityMicromapArrayComputeMemoryUsage)(ctx.raw(), input, &mut sizes)
        })?;
        Ok(MicromapSizes { output: sizes.outputSizeInBytes, temp: sizes.tempSizeInBytes })
    }

    /// Builds an array from bit `data` and host descriptors on `stream`. Every descriptor is
    /// checked (format, level, bits inside `data`) and the histogram is derived from them.
    pub fn build(ctx: &Arc<DeviceContext>, stream: &Arc<Stream>, scratch: &mut Scratch, data: &DeviceBuffer<u8>, descs: &[MicromapDesc], opts: MicromapOptions) -> Result<Arc<Self>> {
        for d in descs {
            let end = u64::from(d.byte_offset).saturating_add(d.bytes()?);
            if end > data.len() as u64 {
                return Err(Error::Invalid("micromap descriptor addresses bits beyond the data buffer"));
            }
        }
        let histogram = histogram_of(descs)?;
        same_ctx(stream, ctx)?;
        // SAFETY: `MicromapDesc` has no padding (asserted above).
        let device = unsafe { upload(stream, descs) }?;
        // SAFETY: descriptors were checked against `data` above and uploaded on `stream`, and
        // the histogram is theirs.
        unsafe { Self::build_raw(ctx, stream, scratch, data, device.cu_deviceptr(), descs.len(), &histogram, opts) }
    }

    /// Builds an array from bit `data` and device-written descriptors counted by `histogram`.
    ///
    /// # Safety
    ///
    /// Every descriptor must have a valid format and level and address bits inside `data`, and
    /// `histogram` must count exactly the descriptors; `data` and `descs` must be written before
    /// (or ordered before) `stream` and stay alive until the build completes on it.
    pub unsafe fn build_unchecked(
        ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        data: &DeviceBuffer<u8>,
        descs: &DeviceBuffer<MicromapDesc>,
        histogram: &[MicromapCount],
        opts: MicromapOptions,
    ) -> Result<Arc<Self>> {
        same_ctx(stream, ctx)?;
        if !descs.cu_deviceptr().is_multiple_of(u64::from(sys::OPTIX_OPACITY_MICROMAP_DESC_BUFFER_BYTE_ALIGNMENT)) {
            return Err(Error::Invalid("micromap descriptors are below OptiX's descriptor alignment"));
        }
        // SAFETY: forwarded contract.
        unsafe { Self::build_raw(ctx, stream, scratch, data, descs.cu_deviceptr(), descs.len(), histogram, opts) }
    }

    /// # Safety
    ///
    /// `descs` points at `n` descriptors valid for `data` and counted by `histogram`, readable in
    /// `stream` order until the build completes.
    #[expect(clippy::too_many_arguments, reason = "private seam shared by the checked and unchecked builds")]
    unsafe fn build_raw(
        ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        data: &DeviceBuffer<u8>,
        descs: sys::CUdeviceptr,
        n: usize,
        histogram: &[MicromapCount],
        opts: MicromapOptions,
    ) -> Result<Arc<Self>> {
        if n == 0 {
            return Err(Error::Invalid("a micromap array needs at least one micromap"));
        }
        let len = count(n, "too many micromaps")?;
        let total = histogram.iter().try_fold(0u32, |acc, h| acc.checked_add(h.count)).ok_or(Error::Invalid("histogram counts overflow u32"))?;
        if total != len {
            return Err(Error::Invalid("histogram does not count exactly the descriptors"));
        }
        // The header asks for a 128-byte aligned data buffer, the array buffer alignment.
        if !data.cu_deviceptr().is_multiple_of(u64::from(sys::OPTIX_OPACITY_MICROMAP_ARRAY_BUFFER_BYTE_ALIGNMENT)) {
            return Err(Error::Invalid("micromap data is below the 128-byte alignment OptiX requires"));
        }
        let entries = histogram.iter().map(MicromapCount::histogram).collect::<Result<Vec<_>>>()?;
        let input = sys::OptixOpacityMicromapArrayBuildInput {
            flags: opts.flags(),
            inputBuffer: data.cu_deviceptr(),
            perMicromapDescBuffer: descs,
            perMicromapDescStrideInBytes: size_of::<MicromapDesc>() as u32,
            numMicromapHistogramEntries: count(entries.len(), "too many histogram entries")?,
            micromapHistogramEntries: entries.as_ptr(),
        };
        let sizes = Self::sizes_raw(ctx, &input)?;
        within(opts.budget, scratch.growth(sizes.temp).saturating_add(sizes.output))?;
        let (temp, temp_len) = scratch.reserve(stream, sizes.temp)?;
        // SAFETY: the build writes the array before anything reads it.
        let buffer = unsafe { alloc(stream, sizes.output, sys::OPTIX_OPACITY_MICROMAP_ARRAY_BUFFER_BYTE_ALIGNMENT) }?;
        let buffers = sys::OptixMicromapBuffers { output: buffer.cu_deviceptr(), outputSizeInBytes: buffer.len(), temp, tempSizeInBytes: temp_len };
        // SAFETY: buffers sized by OptiX and ordered on `stream`; inputs valid per the contract;
        // the histogram outlives the call.
        check("optixOpacityMicromapArrayBuild", unsafe {
            entry!(ctx.table(), optixOpacityMicromapArrayBuild)(ctx.raw(), stream.cu_stream(), &input, &buffers)
        })?;
        Ok(Arc::new(Self { buffer, len, ctx: ctx.clone() }))
    }

    /// Relocation data for this array (`optixOpacityMicromapArrayGetRelocationInfo`).
    pub fn relocation_info(&self) -> Result<RelocationInfo> {
        let mut info = sys::OptixRelocationInfo::default();
        self.ctx.cuda().bind_to_thread()?;
        // SAFETY: owned array of this context; `info` is writable.
        check("optixOpacityMicromapArrayGetRelocationInfo", unsafe {
            entry!(self.ctx.table(), optixOpacityMicromapArrayGetRelocationInfo)(self.ctx.raw(), self.ptr(), &mut info)
        })?;
        Ok(RelocationInfo(info))
    }

    /// A copy of this array in a new buffer on `ctx` (possibly another device), patched by
    /// `optixOpacityMicromapArrayRelocate`. Order: relocation info, compatibility check, budget
    /// check, allocation and copy on `stream`, relocation on `stream`. Structures built over this
    /// array keep using it until relocated with [`crate::accel::Relink::Micromaps`] or refitted.
    ///
    /// # Safety
    ///
    /// The source's build must be complete or ordered before `stream`.
    pub unsafe fn relocate(&self, ctx: &Arc<DeviceContext>, stream: &Arc<Stream>, budget: Option<usize>) -> Result<Arc<Self>> {
        same_ctx(stream, ctx)?;
        let info = self.relocation_info()?;
        if !info.compatible(ctx)? {
            return Err(Error::Invalid("micromap array is not relocatable to this device context"));
        }
        within(budget, self.buffer.len())?;
        // SAFETY: filled by the copy below.
        let target = unsafe { alloc(stream, self.buffer.len(), sys::OPTIX_OPACITY_MICROMAP_ARRAY_BUFFER_BYTE_ALIGNMENT) }?;
        // SAFETY: source writes ordered before `stream` (contract); `target` is fresh.
        unsafe { copy_bytes(stream, &target, &self.buffer) }?;
        ctx.cuda().bind_to_thread()?;
        // SAFETY: `target` holds the copy (same size, array alignment), ordered on `stream`.
        check("optixOpacityMicromapArrayRelocate", unsafe {
            entry!(ctx.table(), optixOpacityMicromapArrayRelocate)(ctx.raw(), stream.cu_stream(), &info.0, target.cu_deviceptr(), target.len())
        })?;
        Ok(Arc::new(Self { buffer: target, len: self.len, ctx: ctx.clone() }))
    }

    /// Device address of the array (for cluster arguments and relocation inputs).
    pub fn ptr(&self) -> sys::CUdeviceptr {
        self.buffer.cu_deviceptr()
    }

    /// Number of micromaps.
    pub fn len(&self) -> u32 {
        self.len
    }

    /// Always false: an array holds at least one micromap.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Bytes the array occupies on the device.
    pub fn size_bytes(&self) -> usize {
        self.buffer.len()
    }

    /// The device context the array was built on.
    pub fn context(&self) -> &Arc<DeviceContext> {
        &self.ctx
    }
}

/// Per-triangle micromap indices (or `OPTIX_OPACITY_MICROMAP_PREDEFINED_INDEX_*` values).
#[derive(Clone, Copy)]
pub enum MicromapIndices<'a> {
    /// 16-bit indices.
    I16(&'a DeviceBuffer<i16>),
    /// 32-bit indices.
    I32(&'a DeviceBuffer<i32>),
}

/// An opacity micromap array applied to one triangle input (`OptixBuildInputOpacityMicromap`).
#[derive(Clone, Copy)]
pub struct MicromapUse<'a> {
    array: &'a Arc<OpacityMicromapArray>,
    indices: Option<(MicromapIndices<'a>, u32)>,
    usage: &'a [MicromapCount],
}

impl<'a> MicromapUse<'a> {
    /// Triangle `i` uses micromap `i` of `array`.
    ///
    /// # Safety
    ///
    /// `usage` must count exactly, per format and level, the micromaps the mesh's triangles use
    /// (OptiX sizes the structure from it).
    pub unsafe fn linear(array: &'a Arc<OpacityMicromapArray>, usage: &'a [MicromapCount]) -> Self {
        Self { array, indices: None, usage }
    }

    /// Triangle `i` uses micromap `indices[i] + offset`, or a predefined state for negative
    /// `OPTIX_OPACITY_MICROMAP_PREDEFINED_INDEX_*` values.
    ///
    /// # Safety
    ///
    /// Every non-negative index plus `offset` must be a micromap of `array`, every negative one a
    /// predefined index, and `usage` must count exactly the micromaps referenced; the indices
    /// must be written before (or ordered before) the build's stream.
    pub unsafe fn indexed(array: &'a Arc<OpacityMicromapArray>, indices: MicromapIndices<'a>, offset: u32, usage: &'a [MicromapCount]) -> Self {
        Self { array, indices: Some((indices, offset)), usage }
    }

    pub(crate) fn array(&self) -> &Arc<OpacityMicromapArray> {
        self.array
    }

    /// The raw input for a mesh of `triangles` triangles (usage pointer left for the caller to
    /// set) and the usage counts it must point at.
    pub(crate) fn input(&self, ctx: &Arc<DeviceContext>, triangles: u32) -> Result<(sys::OptixBuildInputOpacityMicromap, Vec<sys::OptixOpacityMicromapUsageCount>)> {
        if !Arc::ptr_eq(&self.array.ctx, ctx) {
            return Err(Error::Invalid("micromap array from another device context"));
        }
        let usage = self.usage.iter().map(MicromapCount::usage).collect::<Result<Vec<_>>>()?;
        let mut raw = sys::OptixBuildInputOpacityMicromap {
            opacityMicromapArray: self.array.ptr(),
            numMicromapUsageCounts: count(usage.len(), "too many micromap usage counts")?,
            ..Default::default()
        };
        match self.indices {
            None => {
                if self.array.len < triangles {
                    return Err(Error::Invalid("linear micromap indexing needs one micromap per triangle"));
                }
                raw.indexingMode = sys::OptixOpacityMicromapArrayIndexingMode_OPTIX_OPACITY_MICROMAP_ARRAY_INDEXING_MODE_LINEAR;
            }
            Some((indices, offset)) => {
                let (ptr, len, size) = match indices {
                    MicromapIndices::I16(b) => (b.cu_deviceptr(), b.len(), size_of::<i16>() as u32),
                    MicromapIndices::I32(b) => (b.cu_deviceptr(), b.len(), size_of::<i32>() as u32),
                };
                if len != triangles as usize {
                    return Err(Error::Invalid("micromap index buffer length differs from the triangle count"));
                }
                raw.indexingMode = sys::OptixOpacityMicromapArrayIndexingMode_OPTIX_OPACITY_MICROMAP_ARRAY_INDEXING_MODE_INDEXED;
                raw.indexBuffer = ptr;
                raw.indexSizeInBytes = size;
                raw.indexStrideInBytes = size;
                raw.indexOffset = offset;
            }
        }
        Ok((raw, usage))
    }
}

// Port of `optix_impl::extractEvenBits`.
fn extract_even_bits(mut x: u32) -> u32 {
    x &= 0x5555_5555;
    x = (x | (x >> 1)) & 0x3333_3333;
    x = (x | (x >> 2)) & 0x0f0f_0f0f;
    x = (x | (x >> 4)) & 0x00ff_00ff;
    x = (x | (x >> 8)) & 0x0000_ffff;
    x
}

// Port of `optix_impl::prefixEor`: exclusive prefix XOR.
fn prefix_eor(mut x: u32) -> u32 {
    x ^= x >> 1;
    x ^= x >> 2;
    x ^= x >> 4;
    x ^= x >> 8;
    x
}

// Port of `optix_impl::index2dbary`: distance along the space-filling curve to discrete
// barycentrics.
fn index_to_dbary(index: u32) -> (u32, u32, u32) {
    let b0 = extract_even_bits(index);
    let b1 = extract_even_bits(index >> 1);
    let fx = prefix_eor(b0);
    let fy = prefix_eor(b0 & !b1);
    let t = fy ^ b1;
    let u = (fx & !t) | (b0 & !t) | (!b0 & !fx & t);
    let v = fy ^ b0;
    let w = (!fx & !t) | (b0 & !t) | (!b0 & fx & t);
    (u, v, w)
}

/// `optixMicromapIndexToBaseBarycentrics`: the base-triangle barycentrics `[u, v]` of the three
/// vertices of micro-triangle `index` at subdivision `level` (`index < 4^level`, level at most
/// `OPTIX_OPACITY_MICROMAP_MAX_SUBDIVISION_LEVEL`).
pub fn micro_to_base_bary(index: u32, level: u32) -> Result<[[f32; 2]; 3]> {
    let level = check_level(level)?;
    if u64::from(index) >= 1u64 << (2 * level) {
        return Err(Error::Invalid("micro-triangle index out of range of its subdivision level"));
    }
    if level == 0 {
        return Ok([[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]);
    }
    let (iu, iv, iw) = index_to_dbary(index);
    let mask = (1u32 << level) - 1;
    let (iu, iv, iw) = (iu & mask, iv & mask, iw & mask);
    let y_flipped = ((iu & 1) ^ (iv & 1) ^ (iw & 1) ^ 1) as f32;
    // The header's flip pattern; its host build shifts a 64-bit value by `index`, which x86
    // masks to the low 6 bits — `wrapping_shr` reproduces that.
    const FLIP: u64 = 0x8888_8888_8888_8888 ^ 0xf000_f000_f000_f000 ^ 0xffff_0000_0000_0000;
    let x_flipped = ((FLIP.wrapping_shr(index) & 1) ^ (FLIP.wrapping_shr(index >> 6) & 1)) as f32;
    // 2^-level, built from its exponent bits like the header's `__uint_as_float`.
    let scale = f32::from_bits((127 - level) << 23);
    let (u, v) = (iu as f32 * scale, iv as f32 * scale);
    Ok([
        [u + x_flipped * scale, v + y_flipped * scale],
        [u + (1.0 - x_flipped) * scale, v + y_flipped * scale],
        [u + y_flipped * scale, v + (1.0 - y_flipped) * scale],
    ])
}

/// `optixBaseBarycentricsToMicroBarycentrics`: barycentrics of `base` (in the base triangle)
/// relative to the micro-triangle whose vertices sit at `micro` (base barycentrics, as from
/// [`micro_to_base_bary`]). A degenerate micro-triangle yields non-finite values, as in the
/// header.
pub fn base_to_micro_bary(base: [f32; 2], micro: [[f32; 2]; 3]) -> [f32; 2] {
    let [m0, m1, m2] = micro;
    let p = [base[0] - m0[0], base[1] - m0[1]];
    let e1 = [m1[0] - m0[0], m1[1] - m0[1]];
    let e2 = [m2[0] - m0[0], m2[1] - m0[1]];
    let rdet = 1.0 / (e1[0] * e2[1] - e1[1] * e2[0]);
    let a = [e2[1], -e2[0], -e1[1], e1[0]];
    [rdet * (p[0] * a[0] + p[1] * a[1]), rdet * (p[0] * a[2] + p[1] * a[3])]
}
