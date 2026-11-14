//! Texture footprints: which texels a filtered 2D texture fetch would touch, for sparse /
//! virtual texturing (decide which tiles to make resident before sampling).
//!
//! **What.** [`tex_footprint_2d`], [`tex_footprint_2d_grad`] and [`tex_footprint_2d_lod`] are
//! `optixTexFootprint2D`, `optixTexFootprint2DGrad` and `optixTexFootprint2DLod`; each stores a
//! [`Footprint2d`], the header's `Texture2DFootprint` (`uint4`), to caller memory and writes the `singleMipLevel`
//! flag through an out parameter as the header does. [`TexInfo::pack`] builds their `texInfo`
//! argument.
//! **How.** Exactly `internal/optix_device_impl.h`: the float arguments go to the
//! `_optix_tex_footprint_2d*_v2` intrinsics as their bit patterns (`.b32`), `coarse` as `0`/`1`, and
//! the result and flag as 64-bit generic addresses the intrinsic stores to.
//! **Used by** OptiX programs doing texture-space shading or demand-loaded textures; texture handles
//! come from the host crate `cuda-core-ext` (`TextureObject::handle`).

use crate::raw;

/// The footprint of a 2D texture fetch at one mip level (`Texture2DFootprint` in
/// `optix_device.h`, returned as a `uint4`).
///
/// `mask` is an 8x8 bitmask of texel groups covered, or partially covered, by the fetch; bit 0
/// (low order) is group `(tile_x * 8, tile_y * 8)`. A group is `2^granularity`-shaped (for example
/// granularity 12 = 128x64 texels). `dx` / `dy`, when non-zero, rotate the mask toroidally: the
/// last `dx` columns / `dy` rows belong 8 groups earlier. A fetch spanning two mip levels needs
/// two footprint calls (`coarse` false and true).
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Footprint2d {
    /// 8x8 bitmask of covered texel groups.
    pub mask: u64,
    /// Packed `tileY:12, reserved:4, dx:3, dy:3, reserved:2, granularity:4, reserved:4` (LSB first).
    pub word2: u32,
    /// Packed `tileX:12, level:4, reserved:16` (LSB first).
    pub word3: u32,
}

// SAFETY: `repr(C)` of a `u64` and two `u32`s: 16 bytes with no padding (`align(16)` adds none),
// and every bit pattern is a valid value, so all-zero is valid too.
unsafe impl bytemuck::Zeroable for Footprint2d {}
// SAFETY: as above; all fields are `Pod` and the type is `Copy` and `'static`.
unsafe impl bytemuck::Pod for Footprint2d {}

// `Texture2DFootprint` bit fields as `(lowest bit, width)`. The SDK states them only in the
// doc comment of `optixTexFootprint2D`; it has no type or constants for them.
const TILE_Y: (u32, u32) = (0, 12);
const DX: (u32, u32) = (16, 3);
const DY: (u32, u32) = (19, 3);
const GRANULARITY: (u32, u32) = (24, 4);
const TILE_X: (u32, u32) = (0, 12);
const LEVEL: (u32, u32) = (12, 4);

/// Bit field `(lo, n)` of `w`.
#[inline(always)]
const fn field(w: u32, (lo, n): (u32, u32)) -> u32 {
    (w >> lo) & ((1 << n) - 1)
}

impl Footprint2d {
    /// First texel-group tile column of the mask, in 8x8-group blocks.
    #[inline(always)]
    pub const fn tile_x(&self) -> u32 {
        field(self.word3, TILE_X)
    }

    /// First texel-group tile row of the mask, in 8x8-group blocks.
    #[inline(always)]
    pub const fn tile_y(&self) -> u32 {
        field(self.word2, TILE_Y)
    }

    /// Toroidal rotation of the mask in x (0..=7).
    #[inline(always)]
    pub const fn dx(&self) -> u32 {
        field(self.word2, DX)
    }

    /// Toroidal rotation of the mask in y (0..=7).
    #[inline(always)]
    pub const fn dy(&self) -> u32 {
        field(self.word2, DY)
    }

    /// Texel-group size code of the mask; 0 when the requested granularity was honoured.
    #[inline(always)]
    pub const fn granularity(&self) -> u32 {
        field(self.word2, GRANULARITY)
    }

    /// Mip level of this footprint.
    #[inline(always)]
    pub const fn level(&self) -> u32 {
        field(self.word3, LEVEL)
    }
}

/// The texture description a footprint call takes as its packed `texInfo` word.
///
/// Field values are the raw driver enums the texture was created with, as `optix_device.h`
/// specifies; [`TexInfo::pack`] lays them out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TexInfo {
    /// Mip level count (5 bits).
    pub levels: u32,
    /// log2 of the sparse tile width in texels (4 bits).
    pub log2_tile_width: u32,
    /// log2 of the sparse tile height in texels (4 bits).
    pub log2_tile_height: u32,
    /// Horizontal wrap mode, a `CUaddress_mode` value (2 bits).
    pub wrap_x: u32,
    /// Vertical wrap mode, a `CUaddress_mode` value (2 bits).
    pub wrap_y: u32,
    /// Mipmap filter mode, a `CUfilter_mode` value (1 bit).
    pub mip_filter: u32,
    /// Maximum anisotropy (5 bits).
    pub max_anisotropy: u32,
}

// `texInfo` bit fields as `(lowest bit, width)`, from the doc comment of `optixTexFootprint2D`
// (the SDK has no constants for them); bits 31..29 and 15..10 are reserved (zero).
const INFO_LEVELS: (u32, u32) = (24, 5);
const INFO_TILE_W: (u32, u32) = (20, 4);
const INFO_TILE_H: (u32, u32) = (16, 4);
const INFO_WRAP_X: (u32, u32) = (8, 2);
const INFO_WRAP_Y: (u32, u32) = (6, 2);
const INFO_MIP_FILTER: (u32, u32) = (5, 1);
const INFO_ANISO: (u32, u32) = (0, 5);

/// Ors `v` into bit field `(lo, n)` of `w`; `None` once any value has not fit its field.
#[inline(always)]
const fn put(w: Option<u32>, v: u32, (lo, n): (u32, u32)) -> Option<u32> {
    match w {
        Some(w) if v >> n == 0 => Some(w | (v << lo)),
        _ => None,
    }
}

impl TexInfo {
    /// The `texInfo` word for the footprint functions, or `None` when a field exceeds its bit
    /// width (it would corrupt a neighbouring field).
    #[inline(always)]
    pub const fn pack(&self) -> Option<u32> {
        let w = put(Some(0), self.levels, INFO_LEVELS);
        let w = put(w, self.log2_tile_width, INFO_TILE_W);
        let w = put(w, self.log2_tile_height, INFO_TILE_H);
        let w = put(w, self.wrap_x, INFO_WRAP_X);
        let w = put(w, self.wrap_y, INFO_WRAP_Y);
        let w = put(w, self.mip_filter, INFO_MIP_FILTER);
        put(w, self.max_anisotropy, INFO_ANISO)
    }
}

/// `optixTexFootprint2D`: the footprint of a non-mipmapped 2D fetch at `(x, y)`, stored to `out`.
///
/// `tex_info` packs the texture's layout (`optix_device.h`; build it with [`TexInfo::pack`]):
/// bits 28..24 mip level count, 23..20 log2 tile width, 19..16 log2 tile height, 9..8 horizontal
/// and 7..6 vertical `CUaddress_mode`, 5 mip filter (`CUfilter_mode`), 4..0 maximum anisotropy.
/// `single_mip_level` receives non-zero when the footprint spans one mip level.
///
/// The intrinsic stores through addresses, so the result goes to caller memory instead of a
/// returned local: a local would sit in a `.local` depot, which cuda-oxide addresses through
/// 32-bit registers and OptiX programs fault on. Point `out` and `single_mip_level` at global
/// memory (or anything not stack-allocated in the program).
///
/// # Safety
///
/// Valid in every program type (Turing and later). `tex` must be a live texture object over a 2D
/// resource and `tex_info` must describe it.
#[inline(always)]
pub unsafe fn tex_footprint_2d(tex: u64, tex_info: u32, x: f32, y: f32, single_mip_level: &mut u32, out: &mut Footprint2d) {
    let (s, r) = (single_mip_level as *mut u32 as u64, out as *mut Footprint2d as u64);
    // SAFETY: forwarded contract; the intrinsic stores the 16-byte result to `r` (a whole,
    // 16-byte aligned `Footprint2d`) and the flag to `s`.
    unsafe { raw::tex_footprint_2d_v2(tex, tex_info, x.to_bits(), y.to_bits(), s, r) }
}

/// `optixTexFootprint2DGrad`: the footprint of `tex2DGrad` at `(x, y)` with derivatives `dpdx`
/// (`dPdx_x`, `dPdx_y`) and `dpdy` (`dPdy_x`, `dPdy_y`), stored to `out`; `coarse` asks for the
/// coarser of two spanned mip levels.
///
/// # Safety
///
/// As [`tex_footprint_2d`] (`tex` mipmapped as `tex_info` says).
#[inline(always)]
#[expect(clippy::too_many_arguments, reason = "mirrors optixTexFootprint2DGrad's parameter list plus the result")]
pub unsafe fn tex_footprint_2d_grad(
    tex: u64,
    tex_info: u32,
    x: f32,
    y: f32,
    dpdx: [f32; 2],
    dpdy: [f32; 2],
    coarse: bool,
    single_mip_level: &mut u32,
    out: &mut Footprint2d,
) {
    let (s, r) = (single_mip_level as *mut u32 as u64, out as *mut Footprint2d as u64);
    let [dxx, dxy] = dpdx;
    let [dyx, dyy] = dpdy;
    // SAFETY: forwarded contract; the intrinsic stores the 16-byte result to `r` and the flag to `s`.
    unsafe {
        raw::tex_footprint_2d_grad_v2(
            tex,
            tex_info,
            x.to_bits(),
            y.to_bits(),
            dxx.to_bits(),
            dxy.to_bits(),
            dyx.to_bits(),
            dyy.to_bits(),
            u32::from(coarse),
            s,
            r,
        )
    }
}

/// `optixTexFootprint2DLod`: the footprint of `tex2DLod` at `(x, y)` and mip `level`, stored to
/// `out`; `coarse` asks for the coarser of two spanned mip levels.
///
/// # Safety
///
/// As [`tex_footprint_2d`] (`tex` mipmapped as `tex_info` says).
#[inline(always)]
#[expect(clippy::too_many_arguments, reason = "mirrors optixTexFootprint2DLod's parameter list plus the result")]
pub unsafe fn tex_footprint_2d_lod(tex: u64, tex_info: u32, x: f32, y: f32, level: f32, coarse: bool, single_mip_level: &mut u32, out: &mut Footprint2d) {
    let (s, r) = (single_mip_level as *mut u32 as u64, out as *mut Footprint2d as u64);
    // SAFETY: forwarded contract; the intrinsic stores the 16-byte result to `r` and the flag to `s`.
    unsafe { raw::tex_footprint_2d_lod_v2(tex, tex_info, x.to_bits(), y.to_bits(), level.to_bits(), u32::from(coarse), s, r) }
}
