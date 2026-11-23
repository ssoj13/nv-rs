//! Typed shader binding tables.
//!
//! **What.** [`SbtRecord<T>`] is one record: the 32-byte header OptiX packs for a program group
//! followed by user data `T`, which device code reads through `optixGetSbtDataPointer` (the
//! record address plus 32). [`ShaderBindingTable`] uploads the records of a launch (raygen,
//! miss, hit groups, optional callables and exception) and owns the device copies.
//! **Layout.** Each record occupies `size_of::<SbtRecord<T>>()` bytes, a multiple of
//! `OPTIX_SBT_RECORD_ALIGNMENT` (16), which is also the stride OptiX is given. `T: Pod` makes the
//! data bytes well defined; padding after it is written as zeros.

use crate::{Error, ProgramGroup, Result, sys};
use cuda_core::bytemuck::{self, Pod};
use cuda_core::{Context, DeviceBuffer, Stream};

/// `OPTIX_SBT_RECORD_HEADER_SIZE` (a cast macro bindgen does not evaluate).
pub const HEADER: usize = 32;

/// One SBT record: header plus user data.
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct SbtRecord<T: Pod> {
    header: [u8; HEADER],
    /// The data device code sees at `optixGetSbtDataPointer()`.
    pub data: T,
}

// The header ends on a 16-byte boundary, so `data` starts at the address device code computes.
const _: () = assert!(std::mem::offset_of!(SbtRecord<u8>, data) == HEADER);
const _: () = assert!(sys::OPTIX_SBT_RECORD_ALIGNMENT as usize == 16);

impl<T: Pod> SbtRecord<T> {
    /// A record for `group` carrying `data`.
    pub fn new(group: &ProgramGroup, data: T) -> Result<Self> {
        if align_of::<T>() > 16 {
            return Err(Error::Invalid("SBT record data aligned above 16 bytes"));
        }
        let mut header = [0u8; HEADER];
        group.pack_header(&mut header)?;
        Ok(Self { header, data })
    }

    /// Bytes of one record, padding zeroed.
    fn write_into(&self, out: &mut Vec<u8>) {
        let start = out.len();
        out.extend_from_slice(&self.header);
        out.extend_from_slice(bytemuck::bytes_of(&self.data));
        out.resize(start + size_of::<Self>(), 0);
    }
}

/// Uploads `records` as one array; returns the buffer, stride and count.
fn upload<T: Pod>(
    stream: &Stream,
    records: &[SbtRecord<T>],
) -> Result<(DeviceBuffer<u8>, u32, u32)> {
    if records.is_empty() {
        return Err(Error::Invalid("an SBT section needs at least one record"));
    }
    let mut bytes = Vec::with_capacity(size_of_val(records));
    for r in records {
        r.write_into(&mut bytes);
    }
    let stride = u32::try_from(size_of::<SbtRecord<T>>())
        .map_err(|_| Error::Invalid("SBT record too large"))?;
    let count = u32::try_from(records.len()).map_err(|_| Error::Invalid("too many SBT records"))?;
    Ok((DeviceBuffer::from_host(stream, &bytes)?, stride, count))
}

/// The records of a launch, on the device.
pub struct ShaderBindingTable {
    raw: sys::OptixShaderBindingTable,
    buffers: Vec<DeviceBuffer<u8>>,
}

impl ShaderBindingTable {
    /// One raygen record, the miss records (indexed by `optixTrace`'s miss index) and the hit-group
    /// records (indexed by instance SBT offset + mesh index * stride + ray SBT offset).
    pub fn new<R: Pod, M: Pod, H: Pod>(
        stream: &Stream,
        raygen: &SbtRecord<R>,
        miss: &[SbtRecord<M>],
        hit_groups: &[SbtRecord<H>],
    ) -> Result<Self> {
        let (rg, _, _) = upload(stream, std::slice::from_ref(raygen))?;
        let (ms, ms_stride, ms_count) = upload(stream, miss)?;
        let (hg, hg_stride, hg_count) = upload(stream, hit_groups)?;
        let raw = sys::OptixShaderBindingTable {
            raygenRecord: rg.cu_deviceptr(),
            missRecordBase: ms.cu_deviceptr(),
            missRecordStrideInBytes: ms_stride,
            missRecordCount: ms_count,
            hitgroupRecordBase: hg.cu_deviceptr(),
            hitgroupRecordStrideInBytes: hg_stride,
            hitgroupRecordCount: hg_count,
            ..Default::default()
        };
        Ok(Self {
            raw,
            buffers: vec![rg, ms, hg],
        })
    }

    /// Adds callable records (indexed by `optixDirectCall` / `optixContinuationCall`).
    pub fn with_callables<C: Pod>(
        mut self,
        stream: &Stream,
        callables: &[SbtRecord<C>],
    ) -> Result<Self> {
        let (buf, stride, count) = upload(stream, callables)?;
        self.raw.callablesRecordBase = buf.cu_deviceptr();
        self.raw.callablesRecordStrideInBytes = stride;
        self.raw.callablesRecordCount = count;
        self.buffers.push(buf);
        Ok(self)
    }

    /// Adds the exception record.
    pub fn with_exception<E: Pod>(
        mut self,
        stream: &Stream,
        exception: &SbtRecord<E>,
    ) -> Result<Self> {
        let (buf, _, _) = upload(stream, std::slice::from_ref(exception))?;
        self.raw.exceptionRecord = buf.cu_deviceptr();
        self.buffers.push(buf);
        Ok(self)
    }

    /// The raw table.
    pub fn raw(&self) -> &sys::OptixShaderBindingTable {
        &self.raw
    }

    /// Every record buffer lives in `cuda`, so a launch on it reads valid addresses.
    pub(crate) fn on_context(&self, cuda: &Context) -> bool {
        self.buffers
            .iter()
            .all(|b| cuda_core_ext::same_context(b.context(), cuda))
    }
}
