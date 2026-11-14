//! Texture format compatibility table and 256-byte row-pitch alignment helpers.
//!
//! Why: `wgpu` 30 textures and CUDA arrays (`cuda_ext::ArrayDesc` / `ArrayFormat`)
//! only share memory cleanly for specific uncompressed, 1/2/4-channel formats,
//! and buffer-to-texture staging copies in `wgpu` require `256`-byte row-pitch
//! alignment (`wgpu::COPY_BYTES_PER_ROW_ALIGNMENT`). Keeping both in a single
//! authoritative module avoids magic constants and silent format mismatches.
//!
//! Used by `buffer`, `texture`, and user upload/readback paths.

use ash::vk;
use cuda_ext::{ArrayDesc, ArrayFormat};

use crate::error::{Error, Result};

/// Required alignment in bytes for buffer-texture copy row pitches (`bytes_per_row`).
///
/// Mirrors `wgpu::COPY_BYTES_PER_ROW_ALIGNMENT` (`256`).
pub const COPY_BYTES_PER_ROW_ALIGNMENT: u32 = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;

/// Single-source-of-truth mapping between a `wgpu::TextureFormat` and its
/// CUDA array representation plus Vulkan / DXGI native format codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatMapping {
    /// Public `wgpu` texture format.
    pub wgpu: wgpu::TextureFormat,
    /// Corresponding CUDA `CUarray_format` wrapper (`U8`, `U16`, `U32`, `I32`, `F16`, `F32`).
    pub array_format: ArrayFormat,
    /// Number of channels (`1`, `2`, or `4`; CUDA arrays do not support 3-channel formats).
    pub channels: u32,
    /// Total byte width of a single pixel (`array_format.bytes() * channels`).
    pub bytes_per_pixel: usize,
    /// Whether the `wgpu` format applies sRGB transfer encoding.
    pub is_srgb: bool,
    /// Whether channel order is BGRA (swizzled relative to RGBA in raw CUDA memory).
    pub is_bgra: bool,
    /// Vulkan `VkFormat` matching `wgpu`.
    pub vk_format: vk::Format,
    /// DXGI raw format numeric code (`DXGI_FORMAT`), kept platform-independent as `u32`
    /// so tables remain uniform across Windows and Linux tests.
    pub dxgi_format_raw: i32,
}

impl FormatMapping {
    /// Returns the matching [`windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT`].
    #[cfg(windows)]
    #[inline]
    pub const fn dxgi_format(self) -> windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT {
        windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT(self.dxgi_format_raw)
    }
}

/// Authoritative list of all `wgpu::TextureFormat` values supported for CUDA <-> `wgpu` interop.
///
/// Formats absent from this table (compressed BC/ETC2/ASTC, depth/stencil, 3-channel,
/// packed 10-bit, or signed 8/16-bit formats not represented by [`ArrayFormat`])
/// are explicitly rejected by [`format_mapping`] with [`Error::Unsupported`].
pub const SUPPORTED_FORMATS: &[FormatMapping] = &[
    // --- 8-bit unsigned / unorm ---
    FormatMapping {
        wgpu: wgpu::TextureFormat::R8Unorm,
        array_format: ArrayFormat::U8,
        channels: 1,
        bytes_per_pixel: 1,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R8_UNORM,
        dxgi_format_raw: 61, // DXGI_FORMAT_R8_UNORM
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::R8Uint,
        array_format: ArrayFormat::U8,
        channels: 1,
        bytes_per_pixel: 1,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R8_UINT,
        dxgi_format_raw: 62, // DXGI_FORMAT_R8_UINT
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Rg8Unorm,
        array_format: ArrayFormat::U8,
        channels: 2,
        bytes_per_pixel: 2,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R8G8_UNORM,
        dxgi_format_raw: 49, // DXGI_FORMAT_R8G8_UNORM
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Rg8Uint,
        array_format: ArrayFormat::U8,
        channels: 2,
        bytes_per_pixel: 2,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R8G8_UINT,
        dxgi_format_raw: 50, // DXGI_FORMAT_R8G8_UINT
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Rgba8Unorm,
        array_format: ArrayFormat::U8,
        channels: 4,
        bytes_per_pixel: 4,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R8G8B8A8_UNORM,
        dxgi_format_raw: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Rgba8UnormSrgb,
        array_format: ArrayFormat::U8,
        channels: 4,
        bytes_per_pixel: 4,
        is_srgb: true,
        is_bgra: false,
        vk_format: vk::Format::R8G8B8A8_SRGB,
        dxgi_format_raw: 29, // DXGI_FORMAT_R8G8B8A8_UNORM_SRGB
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Rgba8Uint,
        array_format: ArrayFormat::U8,
        channels: 4,
        bytes_per_pixel: 4,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R8G8B8A8_UINT,
        dxgi_format_raw: 30, // DXGI_FORMAT_R8G8B8A8_UINT
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Bgra8Unorm,
        array_format: ArrayFormat::U8,
        channels: 4,
        bytes_per_pixel: 4,
        is_srgb: false,
        is_bgra: true,
        vk_format: vk::Format::B8G8R8A8_UNORM,
        dxgi_format_raw: 87, // DXGI_FORMAT_B8G8R8A8_UNORM
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Bgra8UnormSrgb,
        array_format: ArrayFormat::U8,
        channels: 4,
        bytes_per_pixel: 4,
        is_srgb: true,
        is_bgra: true,
        vk_format: vk::Format::B8G8R8A8_SRGB,
        dxgi_format_raw: 91, // DXGI_FORMAT_B8G8R8A8_UNORM_SRGB
    },
    // --- 16-bit unorm / uint / float ---
    FormatMapping {
        wgpu: wgpu::TextureFormat::R16Unorm,
        array_format: ArrayFormat::U16,
        channels: 1,
        bytes_per_pixel: 2,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R16_UNORM,
        dxgi_format_raw: 56, // DXGI_FORMAT_R16_UNORM
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::R16Uint,
        array_format: ArrayFormat::U16,
        channels: 1,
        bytes_per_pixel: 2,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R16_UINT,
        dxgi_format_raw: 57, // DXGI_FORMAT_R16_UINT
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::R16Float,
        array_format: ArrayFormat::F16,
        channels: 1,
        bytes_per_pixel: 2,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R16_SFLOAT,
        dxgi_format_raw: 54, // DXGI_FORMAT_R16_FLOAT
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Rg16Unorm,
        array_format: ArrayFormat::U16,
        channels: 2,
        bytes_per_pixel: 4,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R16G16_UNORM,
        dxgi_format_raw: 35, // DXGI_FORMAT_R16G16_UNORM
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Rg16Uint,
        array_format: ArrayFormat::U16,
        channels: 2,
        bytes_per_pixel: 4,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R16G16_UINT,
        dxgi_format_raw: 36, // DXGI_FORMAT_R16G16_UINT
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Rg16Float,
        array_format: ArrayFormat::F16,
        channels: 2,
        bytes_per_pixel: 4,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R16G16_SFLOAT,
        dxgi_format_raw: 34, // DXGI_FORMAT_R16G16_FLOAT
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Rgba16Unorm,
        array_format: ArrayFormat::U16,
        channels: 4,
        bytes_per_pixel: 8,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R16G16B16A16_UNORM,
        dxgi_format_raw: 11, // DXGI_FORMAT_R16G16B16A16_UNORM
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Rgba16Uint,
        array_format: ArrayFormat::U16,
        channels: 4,
        bytes_per_pixel: 8,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R16G16B16A16_UINT,
        dxgi_format_raw: 12, // DXGI_FORMAT_R16G16B16A16_UINT
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Rgba16Float,
        array_format: ArrayFormat::F16,
        channels: 4,
        bytes_per_pixel: 8,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R16G16B16A16_SFLOAT,
        dxgi_format_raw: 10, // DXGI_FORMAT_R16G16B16A16_FLOAT
    },
    // --- 32-bit uint / sint / float ---
    FormatMapping {
        wgpu: wgpu::TextureFormat::R32Uint,
        array_format: ArrayFormat::U32,
        channels: 1,
        bytes_per_pixel: 4,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R32_UINT,
        dxgi_format_raw: 42, // DXGI_FORMAT_R32_UINT
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::R32Sint,
        array_format: ArrayFormat::I32,
        channels: 1,
        bytes_per_pixel: 4,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R32_SINT,
        dxgi_format_raw: 43, // DXGI_FORMAT_R32_SINT
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::R32Float,
        array_format: ArrayFormat::F32,
        channels: 1,
        bytes_per_pixel: 4,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R32_SFLOAT,
        dxgi_format_raw: 41, // DXGI_FORMAT_R32_FLOAT
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Rg32Uint,
        array_format: ArrayFormat::U32,
        channels: 2,
        bytes_per_pixel: 8,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R32G32_UINT,
        dxgi_format_raw: 17, // DXGI_FORMAT_R32G32_UINT
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Rg32Sint,
        array_format: ArrayFormat::I32,
        channels: 2,
        bytes_per_pixel: 8,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R32G32_SINT,
        dxgi_format_raw: 18, // DXGI_FORMAT_R32G32_SINT
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Rg32Float,
        array_format: ArrayFormat::F32,
        channels: 2,
        bytes_per_pixel: 8,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R32G32_SFLOAT,
        dxgi_format_raw: 16, // DXGI_FORMAT_R32G32_FLOAT
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Rgba32Uint,
        array_format: ArrayFormat::U32,
        channels: 4,
        bytes_per_pixel: 16,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R32G32B32A32_UINT,
        dxgi_format_raw: 3, // DXGI_FORMAT_R32G32B32A32_UINT
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Rgba32Sint,
        array_format: ArrayFormat::I32,
        channels: 4,
        bytes_per_pixel: 16,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R32G32B32A32_SINT,
        dxgi_format_raw: 4, // DXGI_FORMAT_R32G32B32A32_SINT
    },
    FormatMapping {
        wgpu: wgpu::TextureFormat::Rgba32Float,
        array_format: ArrayFormat::F32,
        channels: 4,
        bytes_per_pixel: 16,
        is_srgb: false,
        is_bgra: false,
        vk_format: vk::Format::R32G32B32A32_SFLOAT,
        dxgi_format_raw: 2, // DXGI_FORMAT_R32G32B32A32_FLOAT
    },
];

/// Looks up the CUDA/Vulkan/DXGI mapping for a [`wgpu::TextureFormat`].
///
/// Returns [`Error::Unsupported`] for compressed, depth/stencil, packed, or
/// signed 8/16-bit formats outside [`SUPPORTED_FORMATS`].
pub fn format_mapping(format: wgpu::TextureFormat) -> Result<FormatMapping> {
    for &entry in SUPPORTED_FORMATS {
        if entry.wgpu == format {
            return Ok(entry);
        }
    }
    Err(Error::Unsupported {
        what: "texture format",
        required: "CUDA-interoperable uncompressed 1/2/4-channel format",
        found: format!("{format:?}"),
    })
}

/// Computes the 256-byte-aligned row pitch in bytes for a texture row of `width` pixels.
///
/// Used when sizing staging buffers for [`wgpu::TexelCopyBufferLayout::bytes_per_row`].
/// Returns `Err(Error::InvalidInput)` if `width == 0`, the format is unsupported,
/// or arithmetic overflows `u32`.
pub fn aligned_row_pitch(format: wgpu::TextureFormat, width: u32) -> Result<u32> {
    if width == 0 {
        return Err(Error::InvalidInput("texture width must be non-zero"));
    }
    let mapping = format_mapping(format)?;
    let bpp = u32::try_from(mapping.bytes_per_pixel)
        .map_err(|_| Error::InvalidInput("bytes_per_pixel exceeds u32"))?;
    let unaligned = width
        .checked_mul(bpp)
        .ok_or(Error::InvalidInput("unaligned row byte size overflows u32"))?;
    align_up_u32(unaligned, COPY_BYTES_PER_ROW_ALIGNMENT)
        .ok_or(Error::InvalidInput("aligned row pitch overflows u32"))
}

/// Computes the minimum total staging buffer byte size for copying `size` texels
/// at `format` with `256`-byte aligned rows.
pub fn aligned_staging_bytes(format: wgpu::TextureFormat, size: wgpu::Extent3d) -> Result<u64> {
    if size.width == 0 || size.height == 0 || size.depth_or_array_layers == 0 {
        return Err(Error::InvalidInput("texture extent dimensions must be non-zero"));
    }
    let row_pitch = u64::from(aligned_row_pitch(format, size.width)?);
    let mapping = format_mapping(format)?;
    let unaligned_row = u64::from(size.width) * (mapping.bytes_per_pixel as u64);
    let slices = u64::from(size.depth_or_array_layers);
    let rows_per_slice = u64::from(size.height);

    // WebGPU requires padded pitch for all rows except the very last row across all slices,
    // which only needs `unaligned_row` bytes when `height * slices > 1`.
    let total_rows = rows_per_slice
        .checked_mul(slices)
        .ok_or(Error::InvalidInput("total row count overflows u64"))?;
    let leading_rows = total_rows.saturating_sub(1);
    let prefix = leading_rows
        .checked_mul(row_pitch)
        .ok_or(Error::InvalidInput("staging buffer prefix size overflows u64"))?;
    prefix
        .checked_add(unaligned_row)
        .ok_or(Error::InvalidInput("total staging buffer size overflows u64"))
}

/// Aligns `value` up to `alignment` (must be a non-zero power of two).
#[inline]
pub const fn align_up_u32(value: u32, alignment: u32) -> Option<u32> {
    if alignment == 0 || !alignment.is_power_of_two() {
        return None;
    }
    let mask = alignment - 1;
    match value.checked_add(mask) {
        Some(added) => Some(added & !mask),
        None => None,
    }
}

/// Aligns `value` up to `alignment` (must be a non-zero power of two).
#[inline]
pub const fn align_up_u64(value: u64, alignment: u64) -> Option<u64> {
    if alignment == 0 || !alignment.is_power_of_two() {
        return None;
    }
    let mask = alignment - 1;
    match value.checked_add(mask) {
        Some(added) => Some(added & !mask),
        None => None,
    }
}

/// Validates a shared texture descriptor and derives the matching CUDA [`ArrayDesc`].
pub fn build_cuda_array_desc(
    size: wgpu::Extent3d,
    dimension: wgpu::TextureDimension,
    format: wgpu::TextureFormat,
    mip_level_count: u32,
    cuda_surface: bool,
    cubemap: bool,
) -> Result<(FormatMapping, ArrayDesc)> {
    if size.width == 0 || size.height == 0 || size.depth_or_array_layers == 0 {
        return Err(Error::InvalidInput("texture extent dimensions must be non-zero"));
    }
    if mip_level_count == 0 {
        return Err(Error::InvalidInput("mip_level_count must be at least 1"));
    }
    let max_dim = match dimension {
        wgpu::TextureDimension::D1 => size.width,
        wgpu::TextureDimension::D2 => size.width.max(size.height),
        wgpu::TextureDimension::D3 => size.width.max(size.height).max(size.depth_or_array_layers),
    };
    let max_mips = u32::BITS - max_dim.leading_zeros();
    if mip_level_count > max_mips {
        return Err(Error::InvalidInput(
            "mip_level_count exceeds maximum mip chain length for extent",
        ));
    }

    let mapping = format_mapping(format)?;
    let (height, depth, layered) = match dimension {
        wgpu::TextureDimension::D1 => {
            if size.height != 1 {
                return Err(Error::InvalidInput("1D texture height must be 1"));
            }
            if size.depth_or_array_layers != 1 {
                return Err(Error::InvalidInput(
                    "1D texture depth_or_array_layers must be 1",
                ));
            }
            if cubemap {
                return Err(Error::InvalidInput("1D texture cannot be a cubemap"));
            }
            (0usize, 1usize, false)
        }
        wgpu::TextureDimension::D2 => {
            if cubemap && (!size.depth_or_array_layers.is_multiple_of(6) || size.width != size.height)
            {
                return Err(Error::InvalidInput(
                    "cubemap texture must be square with depth_or_array_layers divisible by 6",
                ));
            }
            let layers = size.depth_or_array_layers as usize;
            let layered = if cubemap {
                size.depth_or_array_layers > 6
            } else {
                size.depth_or_array_layers > 1
            };
            (size.height as usize, layers.max(1), layered)
        }
        wgpu::TextureDimension::D3 => {
            if cubemap {
                return Err(Error::InvalidInput("3D texture cannot be a cubemap"));
            }
            (
                size.height as usize,
                size.depth_or_array_layers as usize,
                false,
            )
        }
    };

    let desc = ArrayDesc {
        width: size.width as usize,
        height,
        depth,
        format: mapping.array_format,
        channels: mapping.channels,
        surface: cuda_surface,
        layered,
        cubemap,
        sparse: false,
    };

    Ok((mapping, desc))
}

/// Validates a staging buffer copy layout against `wgpu` alignment and bounds rules.
pub fn validate_copy_layout(
    format: wgpu::TextureFormat,
    extent: wgpu::Extent3d,
    bytes_per_row: u32,
    rows_per_image: Option<u32>,
    buffer_size: u64,
    offset: u64,
) -> Result<()> {
    if extent.width == 0 || extent.height == 0 || extent.depth_or_array_layers == 0 {
        return Err(Error::InvalidInput("copy extent dimensions must be non-zero"));
    }
    if !bytes_per_row.is_multiple_of(COPY_BYTES_PER_ROW_ALIGNMENT) {
        return Err(Error::InvalidInput(
            "bytes_per_row must be a multiple of 256 (COPY_BYTES_PER_ROW_ALIGNMENT)",
        ));
    }
    let mapping = format_mapping(format)?;
    let unaligned_row = u64::from(extent.width) * (mapping.bytes_per_pixel as u64);
    if u64::from(bytes_per_row) < unaligned_row {
        return Err(Error::InvalidInput(
            "bytes_per_row is smaller than unaligned texel row byte width",
        ));
    }
    if let Some(rpi) = rows_per_image {
        if rpi < extent.height {
            return Err(Error::InvalidInput(
                "rows_per_image must be >= extent.height",
            ));
        }
    } else if extent.depth_or_array_layers > 1 {
        return Err(Error::InvalidInput(
            "rows_per_image is required when depth_or_array_layers > 1",
        ));
    }
    let rpi = rows_per_image.unwrap_or(extent.height);
    let slices_minus_one = u64::from(extent.depth_or_array_layers - 1);
    let rows_before_last_slice = slices_minus_one
        .checked_mul(u64::from(rpi))
        .ok_or(Error::InvalidInput("row offset overflows u64"))?;
    let total_leading_rows = rows_before_last_slice
        .checked_add(u64::from(extent.height - 1))
        .ok_or(Error::InvalidInput("total leading rows overflows u64"))?;
    let required = total_leading_rows
        .checked_mul(u64::from(bytes_per_row))
        .and_then(|b| b.checked_add(unaligned_row))
        .ok_or(Error::InvalidInput("copy byte range overflows u64"))?;
    let end = offset
        .checked_add(required)
        .ok_or(Error::InvalidInput("buffer offset + size overflows u64"))?;
    if end > buffer_size {
        return Err(Error::InvalidInput(
            "copy layout exceeds staging buffer capacity",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_supported_format_has_consistent_bytes_and_channels() {
        for &entry in SUPPORTED_FORMATS {
            let resolved = format_mapping(entry.wgpu).expect("supported format lookup must succeed");
            assert_eq!(resolved, entry);
            assert!(matches!(entry.channels, 1 | 2 | 4));
            assert_eq!(
                entry.bytes_per_pixel,
                entry.array_format.bytes() * (entry.channels as usize)
            );
            assert_ne!(entry.vk_format, vk::Format::UNDEFINED);
            assert!(entry.dxgi_format_raw > 0);
        }
    }

    #[test]
    fn unsupported_formats_fail_loudly_without_downgrade() {
        let rejected = [
            wgpu::TextureFormat::Depth32Float,
            wgpu::TextureFormat::Depth24PlusStencil8,
            wgpu::TextureFormat::Bc1RgbaUnorm,
            wgpu::TextureFormat::Rgb10a2Unorm,
            wgpu::TextureFormat::R8Snorm,
            wgpu::TextureFormat::Rgba16Snorm,
        ];
        for fmt in rejected {
            let err = format_mapping(fmt).expect_err("format must be explicitly rejected");
            assert!(matches!(
                err,
                Error::Unsupported {
                    what: "texture format",
                    ..
                }
            ));
        }
    }

    #[test]
    fn row_pitch_aligns_to_256_bytes() {
        // 1px RGBA8 = 4B -> 256B
        assert_eq!(aligned_row_pitch(wgpu::TextureFormat::Rgba8Unorm, 1).unwrap(), 256);
        // 64px RGBA8 = 256B -> 256B
        assert_eq!(
            aligned_row_pitch(wgpu::TextureFormat::Rgba8Unorm, 64).unwrap(),
            256
        );
        // 65px RGBA8 = 260B -> 512B
        assert_eq!(
            aligned_row_pitch(wgpu::TextureFormat::Rgba8Unorm, 65).unwrap(),
            512
        );
        // 16px RGBA32Float = 256B -> 256B
        assert_eq!(
            aligned_row_pitch(wgpu::TextureFormat::Rgba32Float, 16).unwrap(),
            256
        );
        // 17px RGBA32Float = 272B -> 512B
        assert_eq!(
            aligned_row_pitch(wgpu::TextureFormat::Rgba32Float, 17).unwrap(),
            512
        );

        assert!(aligned_row_pitch(wgpu::TextureFormat::Rgba8Unorm, 0).is_err());
        assert!(aligned_row_pitch(wgpu::TextureFormat::Rgba32Float, u32::MAX / 4).is_err());
    }

    #[test]
    fn staging_bytes_and_copy_layout_validation() {
        let extent = wgpu::Extent3d {
            width: 3,
            height: 4,
            depth_or_array_layers: 1,
        };
        let bytes = aligned_staging_bytes(wgpu::TextureFormat::Rgba8Unorm, extent).unwrap();
        // 3 leading rows * 256 + last row (3 * 4 = 12) = 780
        assert_eq!(bytes, 3 * 256 + 12);

        validate_copy_layout(
            wgpu::TextureFormat::Rgba8Unorm,
            extent,
            256,
            Some(4),
            bytes,
            0,
        )
        .unwrap();

        // Reject unaligned pitch
        assert!(validate_copy_layout(
            wgpu::TextureFormat::Rgba8Unorm,
            extent,
            128,
            Some(4),
            bytes,
            0,
        )
        .is_err());

        // Reject undersized buffer by 1 byte
        assert!(validate_copy_layout(
            wgpu::TextureFormat::Rgba8Unorm,
            extent,
            256,
            Some(4),
            bytes - 1,
            0,
        )
        .is_err());

        // Reject missing rows_per_image for 2D array / 3D
        let layered = wgpu::Extent3d {
            width: 4,
            height: 4,
            depth_or_array_layers: 2,
        };
        assert!(validate_copy_layout(
            wgpu::TextureFormat::Rgba8Unorm,
            layered,
            256,
            None,
            4096,
            0,
        )
        .is_err());
    }

    #[test]
    fn build_cuda_array_desc_validates_dimensions_and_mips() {
        let (map, d2) = build_cuda_array_desc(
            wgpu::Extent3d {
                width: 64,
                height: 32,
                depth_or_array_layers: 1,
            },
            wgpu::TextureDimension::D2,
            wgpu::TextureFormat::Rgba16Float,
            4,
            true,
            false,
        )
        .unwrap();
        assert_eq!(map.bytes_per_pixel, 8);
        assert_eq!(d2.width, 64);
        assert_eq!(d2.height, 32);
        assert_eq!(d2.depth, 1);
        assert!(d2.surface);
        assert!(!d2.layered);
        assert!(!d2.cubemap);

        // 64x32 has max 7 mips; 8 must fail
        assert!(build_cuda_array_desc(
            wgpu::Extent3d {
                width: 64,
                height: 32,
                depth_or_array_layers: 1,
            },
            wgpu::TextureDimension::D2,
            wgpu::TextureFormat::Rgba8Unorm,
            8,
            false,
            false,
        )
        .is_err());

        // Non-square cubemap must fail
        assert!(build_cuda_array_desc(
            wgpu::Extent3d {
                width: 64,
                height: 32,
                depth_or_array_layers: 6,
            },
            wgpu::TextureDimension::D2,
            wgpu::TextureFormat::Rgba8Unorm,
            1,
            false,
            true,
        )
        .is_err());

        let (_, cube) = build_cuda_array_desc(
            wgpu::Extent3d {
                width: 32,
                height: 32,
                depth_or_array_layers: 6,
            },
            wgpu::TextureDimension::D2,
            wgpu::TextureFormat::Rgba8Unorm,
            1,
            false,
            true,
        )
        .unwrap();
        assert!(cube.cubemap);
        assert!(!cube.layered);
        assert_eq!(cube.depth, 6);

        let (_, cube_array) = build_cuda_array_desc(
            wgpu::Extent3d {
                width: 32,
                height: 32,
                depth_or_array_layers: 12,
            },
            wgpu::TextureDimension::D2,
            wgpu::TextureFormat::Rgba8Unorm,
            1,
            false,
            true,
        )
        .unwrap();
        assert!(cube_array.cubemap);
        assert!(cube_array.layered);
        assert_eq!(cube_array.depth, 12);

        // D1 with depth_or_array_layers != 1 or height != 1 must fail
        assert!(build_cuda_array_desc(
            wgpu::Extent3d {
                width: 64,
                height: 1,
                depth_or_array_layers: 2,
            },
            wgpu::TextureDimension::D1,
            wgpu::TextureFormat::Rgba8Unorm,
            1,
            false,
            false,
        )
        .is_err());

        let (_, d1) = build_cuda_array_desc(
            wgpu::Extent3d {
                width: 64,
                height: 1,
                depth_or_array_layers: 1,
            },
            wgpu::TextureDimension::D1,
            wgpu::TextureFormat::R32Float,
            1,
            false,
            false,
        )
        .unwrap();
        assert_eq!((d1.width, d1.height, d1.depth), (64, 0, 1));
    }
}
