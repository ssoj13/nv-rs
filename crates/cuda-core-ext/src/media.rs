//! Helpers shared by the video and image wrapper crates (`nvenc`, `nvdec`, `nvofa`, `nvjpeg`):
//! 4:2:0 frame geometry, pitched 2D copies, PSNR and the error-detail suffix.
//!
//! **What.**
//! - [`yuv420_rows`] / [`yuv420_frame_bytes`]: the tight geometry of an NV12 / P016 / YV12 / IYUV
//!   frame, where chroma has `ceil(height / 2)` rows (an odd height keeps its last chroma row).
//! - [`copy_2d_device_to_host`] / [`copy_2d_host_to_device`]: one `cuMemcpy2DAsync` between a
//!   pitched device surface and a tightly packed host block, completed before returning.
//! - [`psnr_u8`]: peak signal-to-noise ratio of two 8-bit buffers (codec round-trip checks).
//! - [`detail_suffix`]: the `" (detail)"` tail of a native error message.
//!
//! **Why.** These were copied into several media crates (`MED:X-2`); a fix to one copy (the odd
//! height chroma row) left the others wrong. The lowest crate they all already depend on owns
//! them.
//! **Used by** `nvenc` (`BufferFormat` sizes, error text), `nvdec` (mapped-frame copies, tests),
//! `nvofa` (plane copies, error text) and the `nvjpeg` tests.

use cuda_core::sys;
use cuda_core::{DriverError, IntoResult, Stream};
use std::ffi::c_void;

/// Rows of a tight 4:2:0 frame (planar or semi-planar): `height` luma rows plus
/// `ceil(height / 2)` chroma rows, or `None` on overflow.
pub const fn yuv420_rows(height: u32) -> Option<u32> {
    height.checked_add(height.div_ceil(2))
}

/// Tight byte size of a 4:2:0 frame whose rows are `row_bytes` wide (`width` for 8-bit samples,
/// `2 * width` for 16-bit ones), or `None` on overflow.
pub const fn yuv420_frame_bytes(row_bytes: usize, height: u32) -> Option<usize> {
    match yuv420_rows(height) {
        Some(rows) => row_bytes.checked_mul(rows as usize),
        None => None,
    }
}

/// The `" (detail)"` tail of a native error message, empty when there is no detail.
pub fn detail_suffix(detail: &str) -> String {
    if detail.is_empty() {
        String::new()
    } else {
        format!(" ({detail})")
    }
}

/// Peak signal-to-noise ratio (dB) between two equal-length 8-bit buffers.
///
/// `f64::INFINITY` when identical; `None` when empty or of different lengths.
pub fn psnr_u8(a: &[u8], b: &[u8]) -> Option<f64> {
    if a.is_empty() || a.len() != b.len() {
        return None;
    }
    let mse: f64 = a
        .iter()
        .zip(b.iter())
        .map(|(&x, &y)| {
            let d = f64::from(x) - f64::from(y);
            d * d
        })
        .sum::<f64>()
        / (a.len() as f64);
    if mse == 0.0 {
        Some(f64::INFINITY)
    } else {
        Some(10.0 * (255.0_f64 * 255.0_f64 / mse).log10())
    }
}

/// Enqueues `copy` on `stream` and waits for it. The stream's context is made current first.
///
/// # Safety
///
/// `copy` describes memory that is valid for its extents and stays valid until this returns.
unsafe fn copy_2d(stream: &Stream, copy: &sys::CUDA_MEMCPY2D) -> Result<(), DriverError> {
    stream.context().bind_to_thread()?;
    // SAFETY: forwarded contract; the wait below keeps host memory borrowed until the copy ran.
    unsafe { sys::cuMemcpy2DAsync_v2(copy, stream.cu_stream()) }.result()?;
    stream.synchronize()
}

/// Copies `rows` rows of `width_bytes` from a device surface (`src_pitch` bytes per row) into
/// tightly packed host memory at `dst`, and waits for `stream`.
///
/// # Safety
///
/// `src` addresses `rows` rows of `src_pitch >= width_bytes` bytes in `stream`'s context; `dst` is
/// writable for `width_bytes * rows` bytes; neither is used by another thread meanwhile.
pub unsafe fn copy_2d_device_to_host(
    stream: &Stream,
    src: sys::CUdeviceptr,
    src_pitch: usize,
    dst: *mut c_void,
    width_bytes: usize,
    rows: usize,
) -> Result<(), DriverError> {
    // SAFETY: an all-zero bit pattern is valid for `CUDA_MEMCPY2D`.
    let mut copy: sys::CUDA_MEMCPY2D = unsafe { std::mem::zeroed() };
    copy.srcMemoryType = sys::CUmemorytype_enum_CU_MEMORYTYPE_DEVICE;
    copy.srcDevice = src;
    copy.srcPitch = src_pitch;
    copy.dstMemoryType = sys::CUmemorytype_enum_CU_MEMORYTYPE_HOST;
    copy.dstHost = dst;
    copy.dstPitch = width_bytes;
    copy.WidthInBytes = width_bytes;
    copy.Height = rows;
    // SAFETY: forwarded contract.
    unsafe { copy_2d(stream, &copy) }
}

/// Copies `rows` rows of `width_bytes` from tightly packed host memory at `src` into a device
/// surface (`dst_pitch` bytes per row), and waits for `stream`.
///
/// # Safety
///
/// `src` is readable for `width_bytes * rows` bytes; `dst` addresses `rows` rows of
/// `dst_pitch >= width_bytes` bytes in `stream`'s context; neither is used by another thread
/// meanwhile.
pub unsafe fn copy_2d_host_to_device(
    stream: &Stream,
    src: *const c_void,
    dst: sys::CUdeviceptr,
    dst_pitch: usize,
    width_bytes: usize,
    rows: usize,
) -> Result<(), DriverError> {
    // SAFETY: an all-zero bit pattern is valid for `CUDA_MEMCPY2D`.
    let mut copy: sys::CUDA_MEMCPY2D = unsafe { std::mem::zeroed() };
    copy.srcMemoryType = sys::CUmemorytype_enum_CU_MEMORYTYPE_HOST;
    copy.srcHost = src;
    copy.srcPitch = width_bytes;
    copy.dstMemoryType = sys::CUmemorytype_enum_CU_MEMORYTYPE_DEVICE;
    copy.dstDevice = dst;
    copy.dstPitch = dst_pitch;
    copy.WidthInBytes = width_bytes;
    copy.Height = rows;
    // SAFETY: forwarded contract.
    unsafe { copy_2d(stream, &copy) }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Even heights match the classic `w * h * 3 / 2`; an odd height keeps its last chroma row
    /// (`ceil(h / 2)`), which the old `pixels * 3 / 2` truncated away.
    #[test]
    fn yuv420_geometry_keeps_the_odd_chroma_row() {
        assert_eq!(yuv420_rows(1080), Some(1080 + 540));
        assert_eq!(yuv420_rows(u32::MAX), None, "overflow");
        assert_eq!(yuv420_frame_bytes(1920, 1080), Some(1920 * 1080 * 3 / 2));
        assert_eq!(yuv420_rows(5), Some(5 + 3));
        assert_eq!(yuv420_frame_bytes(4, 5), Some(4 * 8));
        assert_eq!(yuv420_frame_bytes(2 * 4, 5), Some(2 * 4 * 8), "16-bit samples");
        assert_eq!(yuv420_frame_bytes(usize::MAX, 2), None, "overflow");
        assert_eq!(yuv420_rows(0), Some(0));
    }

    #[test]
    fn psnr_u8_behaves_correctly() {
        let a = [100u8; 16];
        assert_eq!(psnr_u8(&a, &a), Some(f64::INFINITY));
        assert_eq!(psnr_u8(&[], &[]), None);
        assert_eq!(psnr_u8(&a, &[100u8; 8]), None);
        let mut b = a;
        b[0] = 101;
        let p = psnr_u8(&a, &b).expect("valid psnr");
        assert!(p > 50.0 && p < 70.0, "unexpected psnr {p}");
    }

    #[test]
    fn detail_suffix_is_empty_or_parenthesised() {
        assert_eq!(detail_suffix(""), "");
        assert_eq!(detail_suffix("bad"), " (bad)");
    }

    /// A tight host block survives a round trip through a pitched device surface, with padding
    /// bytes in the pitch left alone.
    #[test]
    #[ignore = "GPU"]
    fn pitched_copies_round_trip() {
        let ctx = cuda_core::Context::new(0).expect("context");
        let stream = ctx.default_stream();
        let (width, rows, pitch) = (10usize, 6usize, 64usize);
        let tight: Vec<u8> = (0..width * rows).map(|i| (i * 7 + 1) as u8).collect();
        let device = cuda_core::DeviceBuffer::<u8>::from_host(&stream, &vec![0xEEu8; pitch * rows])
            .expect("device surface");
        // SAFETY: `tight` holds `width * rows` bytes; `device` holds `rows` rows of `pitch`.
        unsafe {
            copy_2d_host_to_device(
                &stream,
                tight.as_ptr().cast(),
                device.cu_deviceptr(),
                pitch,
                width,
                rows,
            )
        }
        .expect("upload");
        let mut back = vec![0u8; width * rows];
        // SAFETY: `back` holds `width * rows` bytes; `device` holds `rows` rows of `pitch`.
        unsafe {
            copy_2d_device_to_host(
                &stream,
                device.cu_deviceptr(),
                pitch,
                back.as_mut_ptr().cast(),
                width,
                rows,
            )
        }
        .expect("download");
        assert_eq!(back, tight);
        let raw = device.to_host_vec(&stream).expect("readback");
        assert_eq!(raw[width], 0xEE, "pitch padding is untouched");
    }
}
