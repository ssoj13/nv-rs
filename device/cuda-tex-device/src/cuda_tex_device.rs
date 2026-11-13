//! CUDA texture and surface object reads/writes for Rust device code compiled by cuda-oxide.
//!
//! **Generated** by `gen/gen.py` from its op table (`python gen/gen.py src/cuda_tex_device.rs gen/head.rs`, then
//! `rustfmt --edition 2024`): edit the generator, not this file.
//!
//! **What.** The device half of CUDA's bindless textures and surfaces: every `tex*` fetch of
//! `texture_indirect_functions.h` (`tex1D` .. `texCubemapLayeredGrad`, `tex1Dfetch`,
//! `tex2Dgather`, and the sparse-residency `*_sparse` forms of the `bool* isResident` overloads)
//! and every `surf*read` / `surf*write` of `surface_indirect_functions.h` with all three
//! [`Boundary`] modes, generic over the element type (`tex2d::<[f32; 4]>(tex, x, y)`,
//! `surf2d_write::<[u8; 4]>(v, surf, x, y)`).
//! **Why.** cuda-oxide has no texture intrinsics; environment maps, material textures, sparse
//! (tiled) textures and render targets (OptiX programs and plain kernels alike) need hardware
//! filtering, mipmaps, residency queries and typed image stores.
//! **How.** Each function issues the PTX instruction `nvcc` emits for the CUDA function
//! (`tex.2d.v4.f32.f32`, `tex.level.*`, `tex.grad.*`, `tex.a2d.*`, `tex.cube.*`, `tld4.{r,g,b,a}.2d`,
//! `tex... |p` + `selp.u16` for residency, `suld.b.*.{trap,clamp,zero}`,
//! `sust.b.*.{trap,clamp,zero}`), with the operand layout of clang's
//! `__clang_cuda_texture_intrinsics.h` (its in-header implementation of nvcc's
//! `__nv_tex_surf_handler`), through `cuda_device::ptx_asm!`. Texture fetches always return four
//! channels; the element type keeps the first one, two or four, as CUDA's `__convert` does.
//! Surface functions without `_mode` use CUDA's default boundary mode, `.trap`.
//! **Used by** device crates of nv-rs (OptiX programs and kernels); handles come from the host
//! crate `cuda-ext` (`TextureObject::handle`, `SurfaceObject::handle`).

#![no_std]

use cuda_device::ptx_asm;
use sealed::{Scalar, SurfBits, TexBits};

/// A texture object handle (`cudaTextureObject_t`), from `cuda_ext::TextureObject::handle`.
pub type TexObject = u64;

/// A surface object handle (`cudaSurfaceObject_t`), from `cuda_ext::SurfaceObject::handle`.
pub type SurfObject = u64;

/// What a surface access outside the array does (`cudaSurfaceBoundaryMode`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Boundary {
    /// Abort the launch (`cudaBoundaryModeTrap`, PTX `.trap`); CUDA's default.
    #[default]
    Trap,
    /// Clamp to the edge (`cudaBoundaryModeClamp`, PTX `.clamp`).
    Clamp,
    /// Reads return zero, writes are dropped (`cudaBoundaryModeZero`, PTX `.zero`).
    Zero,
}

mod sealed {
    /// A texture channel type: selects the fetch's PTX destination type (`.f32`, `.s32`, `.u32`).
    pub trait Scalar: Copy {
        /// Four channels of `tex.1d.v4` for this scalar.
        unsafe fn tex1d(t: u64, x: f32) -> [Self; 4];
        /// Four channels of `tex.1d.v4` for this scalar.
        unsafe fn tex1d_fetch(t: u64, x: i32) -> [Self; 4];
        /// Four channels of `tex.level.1d.v4` for this scalar.
        unsafe fn tex1d_lod(t: u64, x: f32, level: f32) -> [Self; 4];
        /// Four channels of `tex.grad.1d.v4` for this scalar.
        unsafe fn tex1d_grad(t: u64, x: f32, dpdx: f32, dpdy: f32) -> [Self; 4];
        /// Four channels of `tex.a1d.v4` for this scalar.
        unsafe fn tex1d_layered(t: u64, x: f32, layer: u32) -> [Self; 4];
        /// Four channels of `tex.level.a1d.v4` for this scalar.
        unsafe fn tex1d_layered_lod(t: u64, x: f32, layer: u32, level: f32) -> [Self; 4];
        /// Four channels of `tex.grad.a1d.v4` for this scalar.
        unsafe fn tex1d_layered_grad(t: u64, x: f32, layer: u32, dpdx: f32, dpdy: f32)
        -> [Self; 4];
        /// Four channels of `tex.2d.v4` for this scalar.
        unsafe fn tex2d(t: u64, x: f32, y: f32) -> [Self; 4];
        /// Four channels of `tex.level.2d.v4` for this scalar.
        unsafe fn tex2d_lod(t: u64, x: f32, y: f32, level: f32) -> [Self; 4];
        /// Four channels of `tex.grad.2d.v4` for this scalar.
        unsafe fn tex2d_grad(t: u64, x: f32, y: f32, dpdx: [f32; 2], dpdy: [f32; 2]) -> [Self; 4];
        /// Four channels of `tex.a2d.v4` for this scalar.
        unsafe fn tex2d_layered(t: u64, x: f32, y: f32, layer: u32) -> [Self; 4];
        /// Four channels of `tex.level.a2d.v4` for this scalar.
        unsafe fn tex2d_layered_lod(t: u64, x: f32, y: f32, layer: u32, level: f32) -> [Self; 4];
        /// Four channels of `tex.grad.a2d.v4` for this scalar.
        unsafe fn tex2d_layered_grad(
            t: u64,
            x: f32,
            y: f32,
            layer: u32,
            dpdx: [f32; 2],
            dpdy: [f32; 2],
        ) -> [Self; 4];
        /// Four channels of `tex.3d.v4` for this scalar.
        unsafe fn tex3d(t: u64, x: f32, y: f32, z: f32) -> [Self; 4];
        /// Four channels of `tex.level.3d.v4` for this scalar.
        unsafe fn tex3d_lod(t: u64, x: f32, y: f32, z: f32, level: f32) -> [Self; 4];
        /// Four channels of `tex.grad.3d.v4` for this scalar.
        unsafe fn tex3d_grad(
            t: u64,
            x: f32,
            y: f32,
            z: f32,
            dpdx: [f32; 3],
            dpdy: [f32; 3],
        ) -> [Self; 4];
        /// Four channels of `tex.cube.v4` for this scalar.
        unsafe fn tex_cube(t: u64, x: f32, y: f32, z: f32) -> [Self; 4];
        /// Four channels of `tex.level.cube.v4` for this scalar.
        unsafe fn tex_cube_lod(t: u64, x: f32, y: f32, z: f32, level: f32) -> [Self; 4];
        /// Four channels of `tex.grad.cube.v4` for this scalar.
        unsafe fn tex_cube_grad(
            t: u64,
            x: f32,
            y: f32,
            z: f32,
            dpdx: [f32; 3],
            dpdy: [f32; 3],
        ) -> [Self; 4];
        /// Four channels of `tex.acube.v4` for this scalar.
        unsafe fn tex_cube_layered(t: u64, x: f32, y: f32, z: f32, layer: u32) -> [Self; 4];
        /// Four channels of `tex.level.acube.v4` for this scalar.
        unsafe fn tex_cube_layered_lod(
            t: u64,
            x: f32,
            y: f32,
            z: f32,
            layer: u32,
            level: f32,
        ) -> [Self; 4];
        /// Four channels of `tex.grad.acube.v4` for this scalar.
        unsafe fn tex_cube_layered_grad(
            t: u64,
            x: f32,
            y: f32,
            z: f32,
            layer: u32,
            dpdx: [f32; 3],
            dpdy: [f32; 3],
        ) -> [Self; 4];
        /// Four channels of `tex.2d.v4` and whether they are resident.
        unsafe fn tex2d_sparse(t: u64, x: f32, y: f32) -> ([Self; 4], bool);
        /// Four channels of `tex.3d.v4` and whether they are resident.
        unsafe fn tex3d_sparse(t: u64, x: f32, y: f32, z: f32) -> ([Self; 4], bool);
        /// Four channels of `tex.a2d.v4` and whether they are resident.
        unsafe fn tex2d_layered_sparse(t: u64, x: f32, y: f32, layer: u32) -> ([Self; 4], bool);
        /// Four channels of `tex.level.2d.v4` and whether they are resident.
        unsafe fn tex2d_lod_sparse(t: u64, x: f32, y: f32, level: f32) -> ([Self; 4], bool);
        /// Four channels of `tex.level.3d.v4` and whether they are resident.
        unsafe fn tex3d_lod_sparse(t: u64, x: f32, y: f32, z: f32, level: f32)
        -> ([Self; 4], bool);
        /// Four channels of `tex.level.a2d.v4` and whether they are resident.
        unsafe fn tex2d_layered_lod_sparse(
            t: u64,
            x: f32,
            y: f32,
            layer: u32,
            level: f32,
        ) -> ([Self; 4], bool);
        /// Four channels of `tex.grad.2d.v4` and whether they are resident.
        unsafe fn tex2d_grad_sparse(
            t: u64,
            x: f32,
            y: f32,
            dpdx: [f32; 2],
            dpdy: [f32; 2],
        ) -> ([Self; 4], bool);
        /// Four channels of `tex.grad.3d.v4` and whether they are resident.
        unsafe fn tex3d_grad_sparse(
            t: u64,
            x: f32,
            y: f32,
            z: f32,
            dpdx: [f32; 3],
            dpdy: [f32; 3],
        ) -> ([Self; 4], bool);
        /// Four channels of `tex.grad.a2d.v4` and whether they are resident.
        unsafe fn tex2d_layered_grad_sparse(
            t: u64,
            x: f32,
            y: f32,
            layer: u32,
            dpdx: [f32; 2],
            dpdy: [f32; 2],
        ) -> ([Self; 4], bool);
        /// One channel of four texels (`tld4.r.2d.v4`) for this scalar.
        unsafe fn gather_r(t: u64, x: f32, y: f32) -> [Self; 4];
        /// As `gather_r`, and whether they are resident.
        unsafe fn gather_r_sparse(t: u64, x: f32, y: f32) -> ([Self; 4], bool);
        /// One channel of four texels (`tld4.g.2d.v4`) for this scalar.
        unsafe fn gather_g(t: u64, x: f32, y: f32) -> [Self; 4];
        /// As `gather_g`, and whether they are resident.
        unsafe fn gather_g_sparse(t: u64, x: f32, y: f32) -> ([Self; 4], bool);
        /// One channel of four texels (`tld4.b.2d.v4`) for this scalar.
        unsafe fn gather_b(t: u64, x: f32, y: f32) -> [Self; 4];
        /// As `gather_b`, and whether they are resident.
        unsafe fn gather_b_sparse(t: u64, x: f32, y: f32) -> ([Self; 4], bool);
        /// One channel of four texels (`tld4.a.2d.v4`) for this scalar.
        unsafe fn gather_a(t: u64, x: f32, y: f32) -> [Self; 4];
        /// As `gather_a`, and whether they are resident.
        unsafe fn gather_a_sparse(t: u64, x: f32, y: f32) -> ([Self; 4], bool);
    }

    /// How a texture element is cut from the four fetched channels.
    pub trait TexBits: Copy {
        /// The channel type.
        type S: Scalar;
        /// Keeps the channels this element holds.
        fn from_v4(v: [Self::S; 4]) -> Self;
    }

    /// How a surface element maps to PTX `suld.b` / `sust.b` components.
    pub trait SurfBits: Copy {
        /// Component width in bits: 8, 16, 32 or 64.
        const BITS: u32;
        /// Components per element: 1, 2 or 4 (64-bit: 1 or 2).
        const N: usize;
        /// The components' bits, zero-extended (unused tail entries are zero).
        fn to_bits(self) -> [u64; 4];
        /// The element from its first `N` components' bits (high bits ignored).
        fn from_bits(w: [u64; 4]) -> Self;
    }
}

/// A texture fetch result: `f32`, `i32` or `u32`, alone or as `[_; 2]` / `[_; 4]` (CUDA's
/// `float`/`float2`/`float4`, `int*`, `uint*`).
///
/// `f32` elements read float formats, and integer formats the host created without
/// `read_as_integer` (8/16-bit normalized to `[0, 1]` / `[-1, 1]`, filterable). `i32` / `u32`
/// elements read integer formats created with `read_as_integer` (unfiltered, raw values).
/// Fetching with a channel type that does not match the texture's read mode is undefined.
pub trait TexElem: TexBits {}

/// A surface element, as CUDA's `surf*read` / `surf*write` accept: `i8`/`u8`, `i16`/`u16`,
/// `i32`/`u32`/`f32` alone or as `[_; 2]` / `[_; 4]`; `i64`/`u64` alone or as `[_; 2]`; plus
/// `f64` / `[f64; 2]` (a `b64` bit copy; CUDA itself has no `double` surface overload).
///
/// Surfaces move raw bits (`suld.b` / `sust.b`), so the element's size must equal the array
/// element's size and surface `x` coordinates are byte offsets (`index * size_of::<T>()`), as
/// in CUDA.
pub trait SurfElem: SurfBits {}

macro_rules! tex_elem {
    ($($s:ty),*) => {$(
        impl TexBits for $s {
            type S = $s;
            #[inline(always)]
            fn from_v4(v: [$s; 4]) -> Self {
                v[0]
            }
        }
        impl TexElem for $s {}
        impl TexBits for [$s; 2] {
            type S = $s;
            #[inline(always)]
            fn from_v4(v: [$s; 4]) -> Self {
                [v[0], v[1]]
            }
        }
        impl TexElem for [$s; 2] {}
        impl TexBits for [$s; 4] {
            type S = $s;
            #[inline(always)]
            fn from_v4(v: [$s; 4]) -> Self {
                v
            }
        }
        impl TexElem for [$s; 4] {}
    )*};
}
tex_elem!(f32, i32, u32);

impl SurfBits for i8 {
    const BITS: u32 = 8;
    const N: usize = 1;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [u64::from(self as u8), 0, 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        w[0] as u8 as i8
    }
}
impl SurfElem for i8 {}

impl SurfBits for [i8; 2] {
    const BITS: u32 = 8;
    const N: usize = 2;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [u64::from(self[0] as u8), u64::from(self[1] as u8), 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        [w[0] as u8 as i8, w[1] as u8 as i8]
    }
}
impl SurfElem for [i8; 2] {}

impl SurfBits for [i8; 4] {
    const BITS: u32 = 8;
    const N: usize = 4;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [
            u64::from(self[0] as u8),
            u64::from(self[1] as u8),
            u64::from(self[2] as u8),
            u64::from(self[3] as u8),
        ]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        [
            w[0] as u8 as i8,
            w[1] as u8 as i8,
            w[2] as u8 as i8,
            w[3] as u8 as i8,
        ]
    }
}
impl SurfElem for [i8; 4] {}

impl SurfBits for u8 {
    const BITS: u32 = 8;
    const N: usize = 1;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [u64::from(self), 0, 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        w[0] as u8
    }
}
impl SurfElem for u8 {}

impl SurfBits for [u8; 2] {
    const BITS: u32 = 8;
    const N: usize = 2;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [u64::from(self[0]), u64::from(self[1]), 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        [w[0] as u8, w[1] as u8]
    }
}
impl SurfElem for [u8; 2] {}

impl SurfBits for [u8; 4] {
    const BITS: u32 = 8;
    const N: usize = 4;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [
            u64::from(self[0]),
            u64::from(self[1]),
            u64::from(self[2]),
            u64::from(self[3]),
        ]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        [w[0] as u8, w[1] as u8, w[2] as u8, w[3] as u8]
    }
}
impl SurfElem for [u8; 4] {}

impl SurfBits for i16 {
    const BITS: u32 = 16;
    const N: usize = 1;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [u64::from(self as u16), 0, 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        w[0] as u16 as i16
    }
}
impl SurfElem for i16 {}

impl SurfBits for [i16; 2] {
    const BITS: u32 = 16;
    const N: usize = 2;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [u64::from(self[0] as u16), u64::from(self[1] as u16), 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        [w[0] as u16 as i16, w[1] as u16 as i16]
    }
}
impl SurfElem for [i16; 2] {}

impl SurfBits for [i16; 4] {
    const BITS: u32 = 16;
    const N: usize = 4;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [
            u64::from(self[0] as u16),
            u64::from(self[1] as u16),
            u64::from(self[2] as u16),
            u64::from(self[3] as u16),
        ]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        [
            w[0] as u16 as i16,
            w[1] as u16 as i16,
            w[2] as u16 as i16,
            w[3] as u16 as i16,
        ]
    }
}
impl SurfElem for [i16; 4] {}

impl SurfBits for u16 {
    const BITS: u32 = 16;
    const N: usize = 1;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [u64::from(self), 0, 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        w[0] as u16
    }
}
impl SurfElem for u16 {}

impl SurfBits for [u16; 2] {
    const BITS: u32 = 16;
    const N: usize = 2;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [u64::from(self[0]), u64::from(self[1]), 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        [w[0] as u16, w[1] as u16]
    }
}
impl SurfElem for [u16; 2] {}

impl SurfBits for [u16; 4] {
    const BITS: u32 = 16;
    const N: usize = 4;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [
            u64::from(self[0]),
            u64::from(self[1]),
            u64::from(self[2]),
            u64::from(self[3]),
        ]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        [w[0] as u16, w[1] as u16, w[2] as u16, w[3] as u16]
    }
}
impl SurfElem for [u16; 4] {}

impl SurfBits for i32 {
    const BITS: u32 = 32;
    const N: usize = 1;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [u64::from(self as u32), 0, 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        w[0] as u32 as i32
    }
}
impl SurfElem for i32 {}

impl SurfBits for [i32; 2] {
    const BITS: u32 = 32;
    const N: usize = 2;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [u64::from(self[0] as u32), u64::from(self[1] as u32), 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        [w[0] as u32 as i32, w[1] as u32 as i32]
    }
}
impl SurfElem for [i32; 2] {}

impl SurfBits for [i32; 4] {
    const BITS: u32 = 32;
    const N: usize = 4;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [
            u64::from(self[0] as u32),
            u64::from(self[1] as u32),
            u64::from(self[2] as u32),
            u64::from(self[3] as u32),
        ]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        [
            w[0] as u32 as i32,
            w[1] as u32 as i32,
            w[2] as u32 as i32,
            w[3] as u32 as i32,
        ]
    }
}
impl SurfElem for [i32; 4] {}

impl SurfBits for u32 {
    const BITS: u32 = 32;
    const N: usize = 1;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [u64::from(self), 0, 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        w[0] as u32
    }
}
impl SurfElem for u32 {}

impl SurfBits for [u32; 2] {
    const BITS: u32 = 32;
    const N: usize = 2;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [u64::from(self[0]), u64::from(self[1]), 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        [w[0] as u32, w[1] as u32]
    }
}
impl SurfElem for [u32; 2] {}

impl SurfBits for [u32; 4] {
    const BITS: u32 = 32;
    const N: usize = 4;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [
            u64::from(self[0]),
            u64::from(self[1]),
            u64::from(self[2]),
            u64::from(self[3]),
        ]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        [w[0] as u32, w[1] as u32, w[2] as u32, w[3] as u32]
    }
}
impl SurfElem for [u32; 4] {}

impl SurfBits for f32 {
    const BITS: u32 = 32;
    const N: usize = 1;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [u64::from(self.to_bits()), 0, 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        f32::from_bits(w[0] as u32)
    }
}
impl SurfElem for f32 {}

impl SurfBits for [f32; 2] {
    const BITS: u32 = 32;
    const N: usize = 2;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [
            u64::from(self[0].to_bits()),
            u64::from(self[1].to_bits()),
            0,
            0,
        ]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        [f32::from_bits(w[0] as u32), f32::from_bits(w[1] as u32)]
    }
}
impl SurfElem for [f32; 2] {}

impl SurfBits for [f32; 4] {
    const BITS: u32 = 32;
    const N: usize = 4;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [
            u64::from(self[0].to_bits()),
            u64::from(self[1].to_bits()),
            u64::from(self[2].to_bits()),
            u64::from(self[3].to_bits()),
        ]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        [
            f32::from_bits(w[0] as u32),
            f32::from_bits(w[1] as u32),
            f32::from_bits(w[2] as u32),
            f32::from_bits(w[3] as u32),
        ]
    }
}
impl SurfElem for [f32; 4] {}

impl SurfBits for i64 {
    const BITS: u32 = 64;
    const N: usize = 1;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [self as u64, 0, 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        w[0] as i64
    }
}
impl SurfElem for i64 {}

impl SurfBits for [i64; 2] {
    const BITS: u32 = 64;
    const N: usize = 2;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [self[0] as u64, self[1] as u64, 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        [w[0] as i64, w[1] as i64]
    }
}
impl SurfElem for [i64; 2] {}

impl SurfBits for u64 {
    const BITS: u32 = 64;
    const N: usize = 1;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [self, 0, 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        w[0]
    }
}
impl SurfElem for u64 {}

impl SurfBits for [u64; 2] {
    const BITS: u32 = 64;
    const N: usize = 2;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [self[0], self[1], 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        [w[0], w[1]]
    }
}
impl SurfElem for [u64; 2] {}

impl SurfBits for f64 {
    const BITS: u32 = 64;
    const N: usize = 1;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [self.to_bits(), 0, 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        f64::from_bits(w[0])
    }
}
impl SurfElem for f64 {}

impl SurfBits for [f64; 2] {
    const BITS: u32 = 64;
    const N: usize = 2;
    #[inline(always)]
    fn to_bits(self) -> [u64; 4] {
        [self[0].to_bits(), self[1].to_bits(), 0, 0]
    }
    #[inline(always)]
    fn from_bits(w: [u64; 4]) -> Self {
        [f64::from_bits(w[0]), f64::from_bits(w[1])]
    }
}
impl SurfElem for [f64; 2] {}

impl sealed::Scalar for f32 {
    /// PTX `tex.1d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex1d(t: u64, x: f32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.1d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5}];", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("f") x, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.1d.v4.f32.s32`.
    #[inline(always)]
    unsafe fn tex1d_fetch(t: u64, x: i32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.1d.v4.f32.s32 {%0, %1, %2, %3}, [%4, {%5}];", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("r") x, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.1d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex1d_lod(t: u64, x: f32, level: f32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.1d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5}], %6;", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("f") x, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.1d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex1d_grad(t: u64, x: f32, dpdx: f32, dpdy: f32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.1d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5}], {%6}, {%7};", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("f") x, in("f") dpdx, in("f") dpdy, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.a1d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex1d_layered(t: u64, x: f32, layer: u32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.a1d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("r") layer, in("f") x, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.a1d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex1d_layered_lod(t: u64, x: f32, layer: u32, level: f32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.a1d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}], %7;", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("r") layer, in("f") x, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.a1d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex1d_layered_grad(t: u64, x: f32, layer: u32, dpdx: f32, dpdy: f32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.a1d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}], {%7}, {%8};", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("r") layer, in("f") x, in("f") dpdx, in("f") dpdy, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.2d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex2d(t: u64, x: f32, y: f32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.2d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.2d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex2d_lod(t: u64, x: f32, y: f32, level: f32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.2d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}], %7;", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("f") x, in("f") y, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.2d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex2d_grad(t: u64, x: f32, y: f32, dpdx: [f32; 2], dpdy: [f32; 2]) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.2d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}], {%7, %8}, {%9, %10};", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("f") x, in("f") y, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdy[0], in("f") dpdy[1], options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.a2d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex2d_layered(t: u64, x: f32, y: f32, layer: u32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.a2d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("r") layer, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.a2d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex2d_layered_lod(t: u64, x: f32, y: f32, layer: u32, level: f32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.a2d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], %8;", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.a2d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex2d_layered_grad(
        t: u64,
        x: f32,
        y: f32,
        layer: u32,
        dpdx: [f32; 2],
        dpdy: [f32; 2],
    ) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.a2d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], {%8, %9}, {%10, %11};", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdy[0], in("f") dpdy[1], options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.3d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex3d(t: u64, x: f32, y: f32, z: f32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.3d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("f") x, in("f") y, in("f") z, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.3d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex3d_lod(t: u64, x: f32, y: f32, z: f32, level: f32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.3d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], %8;", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("f") x, in("f") y, in("f") z, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.3d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex3d_grad(
        t: u64,
        x: f32,
        y: f32,
        z: f32,
        dpdx: [f32; 3],
        dpdy: [f32; 3],
    ) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.3d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], {%8, %9, %10, %10}, {%11, %12, %13, %13};", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("f") x, in("f") y, in("f") z, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdx[2], in("f") dpdy[0], in("f") dpdy[1], in("f") dpdy[2], options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.cube.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex_cube(t: u64, x: f32, y: f32, z: f32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.cube.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("f") x, in("f") y, in("f") z, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.cube.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex_cube_lod(t: u64, x: f32, y: f32, z: f32, level: f32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.cube.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], %8;", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("f") x, in("f") y, in("f") z, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.cube.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex_cube_grad(
        t: u64,
        x: f32,
        y: f32,
        z: f32,
        dpdx: [f32; 3],
        dpdy: [f32; 3],
    ) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.cube.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], {%8, %9, %10, %10}, {%11, %12, %13, %13};", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("f") x, in("f") y, in("f") z, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdx[2], in("f") dpdy[0], in("f") dpdy[1], in("f") dpdy[2], options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.acube.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex_cube_layered(t: u64, x: f32, y: f32, z: f32, layer: u32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.acube.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %8}];", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") z, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.acube.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex_cube_layered_lod(
        t: u64,
        x: f32,
        y: f32,
        z: f32,
        layer: u32,
        level: f32,
    ) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.acube.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %8}], %9;", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") z, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.acube.v4.f32.f32`.
    #[inline(always)]
    unsafe fn tex_cube_layered_grad(
        t: u64,
        x: f32,
        y: f32,
        z: f32,
        layer: u32,
        dpdx: [f32; 3],
        dpdy: [f32; 3],
    ) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.acube.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %8}], {%9, %10, %11, %11}, {%12, %13, %14, %14};", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") z, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdx[2], in("f") dpdy[0], in("f") dpdy[1], in("f") dpdy[2], options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.2d.v4.f32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_sparse(t: u64, x: f32, y: f32) -> ([f32; 4], bool) {
        let (a, b, c, d): (f32, f32, f32, f32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.2d.v4.f32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=f") a, out("=f") b, out("=f") c, out("=f") d, out("=h") r, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.3d.v4.f32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex3d_sparse(t: u64, x: f32, y: f32, z: f32) -> ([f32; 4], bool) {
        let (a, b, c, d): (f32, f32, f32, f32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.3d.v4.f32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=f") a, out("=f") b, out("=f") c, out("=f") d, out("=h") r, in("l") t, in("f") x, in("f") y, in("f") z, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.a2d.v4.f32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_layered_sparse(t: u64, x: f32, y: f32, layer: u32) -> ([f32; 4], bool) {
        let (a, b, c, d): (f32, f32, f32, f32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.a2d.v4.f32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=f") a, out("=f") b, out("=f") c, out("=f") d, out("=h") r, in("l") t, in("r") layer, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.level.2d.v4.f32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_lod_sparse(t: u64, x: f32, y: f32, level: f32) -> ([f32; 4], bool) {
        let (a, b, c, d): (f32, f32, f32, f32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.level.2d.v4.f32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}], %8;\n\tselp.u16 %4, 1, 0, %%p0; }", out("=f") a, out("=f") b, out("=f") c, out("=f") d, out("=h") r, in("l") t, in("f") x, in("f") y, in("f") level, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.level.3d.v4.f32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex3d_lod_sparse(t: u64, x: f32, y: f32, z: f32, level: f32) -> ([f32; 4], bool) {
        let (a, b, c, d): (f32, f32, f32, f32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.level.3d.v4.f32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}], %9;\n\tselp.u16 %4, 1, 0, %%p0; }", out("=f") a, out("=f") b, out("=f") c, out("=f") d, out("=h") r, in("l") t, in("f") x, in("f") y, in("f") z, in("f") level, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.level.a2d.v4.f32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_layered_lod_sparse(
        t: u64,
        x: f32,
        y: f32,
        layer: u32,
        level: f32,
    ) -> ([f32; 4], bool) {
        let (a, b, c, d): (f32, f32, f32, f32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.level.a2d.v4.f32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}], %9;\n\tselp.u16 %4, 1, 0, %%p0; }", out("=f") a, out("=f") b, out("=f") c, out("=f") d, out("=h") r, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") level, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.grad.2d.v4.f32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_grad_sparse(
        t: u64,
        x: f32,
        y: f32,
        dpdx: [f32; 2],
        dpdy: [f32; 2],
    ) -> ([f32; 4], bool) {
        let (a, b, c, d): (f32, f32, f32, f32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.grad.2d.v4.f32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}], {%8, %9}, {%10, %11};\n\tselp.u16 %4, 1, 0, %%p0; }", out("=f") a, out("=f") b, out("=f") c, out("=f") d, out("=h") r, in("l") t, in("f") x, in("f") y, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdy[0], in("f") dpdy[1], options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.grad.3d.v4.f32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex3d_grad_sparse(
        t: u64,
        x: f32,
        y: f32,
        z: f32,
        dpdx: [f32; 3],
        dpdy: [f32; 3],
    ) -> ([f32; 4], bool) {
        let (a, b, c, d): (f32, f32, f32, f32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.grad.3d.v4.f32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}], {%9, %10, %11, %11}, {%12, %13, %14, %14};\n\tselp.u16 %4, 1, 0, %%p0; }", out("=f") a, out("=f") b, out("=f") c, out("=f") d, out("=h") r, in("l") t, in("f") x, in("f") y, in("f") z, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdx[2], in("f") dpdy[0], in("f") dpdy[1], in("f") dpdy[2], options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.grad.a2d.v4.f32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_layered_grad_sparse(
        t: u64,
        x: f32,
        y: f32,
        layer: u32,
        dpdx: [f32; 2],
        dpdy: [f32; 2],
    ) -> ([f32; 4], bool) {
        let (a, b, c, d): (f32, f32, f32, f32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.grad.a2d.v4.f32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}], {%9, %10}, {%11, %12};\n\tselp.u16 %4, 1, 0, %%p0; }", out("=f") a, out("=f") b, out("=f") c, out("=f") d, out("=h") r, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdy[0], in("f") dpdy[1], options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tld4.r.2d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn gather_r(t: u64, x: f32, y: f32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tld4.r.2d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tld4.r.2d.v4.f32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn gather_r_sparse(t: u64, x: f32, y: f32) -> ([f32; 4], bool) {
        let (a, b, c, d): (f32, f32, f32, f32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttld4.r.2d.v4.f32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=f") a, out("=f") b, out("=f") c, out("=f") d, out("=h") r, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tld4.g.2d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn gather_g(t: u64, x: f32, y: f32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tld4.g.2d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tld4.g.2d.v4.f32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn gather_g_sparse(t: u64, x: f32, y: f32) -> ([f32; 4], bool) {
        let (a, b, c, d): (f32, f32, f32, f32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttld4.g.2d.v4.f32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=f") a, out("=f") b, out("=f") c, out("=f") d, out("=h") r, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tld4.b.2d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn gather_b(t: u64, x: f32, y: f32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tld4.b.2d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tld4.b.2d.v4.f32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn gather_b_sparse(t: u64, x: f32, y: f32) -> ([f32; 4], bool) {
        let (a, b, c, d): (f32, f32, f32, f32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttld4.b.2d.v4.f32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=f") a, out("=f") b, out("=f") c, out("=f") d, out("=h") r, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tld4.a.2d.v4.f32.f32`.
    #[inline(always)]
    unsafe fn gather_a(t: u64, x: f32, y: f32) -> [f32; 4] {
        let (a, b, c, d): (f32, f32, f32, f32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tld4.a.2d.v4.f32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=f") a, out("=f") b, out("=f") c, out("=f") d, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tld4.a.2d.v4.f32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn gather_a_sparse(t: u64, x: f32, y: f32) -> ([f32; 4], bool) {
        let (a, b, c, d): (f32, f32, f32, f32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttld4.a.2d.v4.f32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=f") a, out("=f") b, out("=f") c, out("=f") d, out("=h") r, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
}

impl sealed::Scalar for i32 {
    /// PTX `tex.1d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex1d(t: u64, x: f32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.1d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.1d.v4.s32.s32`.
    #[inline(always)]
    unsafe fn tex1d_fetch(t: u64, x: i32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.1d.v4.s32.s32 {%0, %1, %2, %3}, [%4, {%5}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") x, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.1d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex1d_lod(t: u64, x: f32, level: f32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.1d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5}], %6;", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.1d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex1d_grad(t: u64, x: f32, dpdx: f32, dpdy: f32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.1d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5}], {%6}, {%7};", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") dpdx, in("f") dpdy, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.a1d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex1d_layered(t: u64, x: f32, layer: u32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.a1d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.a1d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex1d_layered_lod(t: u64, x: f32, layer: u32, level: f32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.a1d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}], %7;", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.a1d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex1d_layered_grad(t: u64, x: f32, layer: u32, dpdx: f32, dpdy: f32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.a1d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}], {%7}, {%8};", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, in("f") dpdx, in("f") dpdy, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.2d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex2d(t: u64, x: f32, y: f32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.2d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.2d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex2d_lod(t: u64, x: f32, y: f32, level: f32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.2d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}], %7;", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.2d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex2d_grad(t: u64, x: f32, y: f32, dpdx: [f32; 2], dpdy: [f32; 2]) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.2d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}], {%7, %8}, {%9, %10};", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdy[0], in("f") dpdy[1], options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.a2d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex2d_layered(t: u64, x: f32, y: f32, layer: u32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.a2d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.a2d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex2d_layered_lod(t: u64, x: f32, y: f32, layer: u32, level: f32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.a2d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], %8;", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.a2d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex2d_layered_grad(
        t: u64,
        x: f32,
        y: f32,
        layer: u32,
        dpdx: [f32; 2],
        dpdy: [f32; 2],
    ) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.a2d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], {%8, %9}, {%10, %11};", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdy[0], in("f") dpdy[1], options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.3d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex3d(t: u64, x: f32, y: f32, z: f32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.3d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, in("f") z, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.3d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex3d_lod(t: u64, x: f32, y: f32, z: f32, level: f32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.3d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], %8;", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, in("f") z, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.3d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex3d_grad(
        t: u64,
        x: f32,
        y: f32,
        z: f32,
        dpdx: [f32; 3],
        dpdy: [f32; 3],
    ) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.3d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], {%8, %9, %10, %10}, {%11, %12, %13, %13};", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, in("f") z, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdx[2], in("f") dpdy[0], in("f") dpdy[1], in("f") dpdy[2], options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.cube.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex_cube(t: u64, x: f32, y: f32, z: f32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.cube.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, in("f") z, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.cube.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex_cube_lod(t: u64, x: f32, y: f32, z: f32, level: f32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.cube.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], %8;", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, in("f") z, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.cube.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex_cube_grad(
        t: u64,
        x: f32,
        y: f32,
        z: f32,
        dpdx: [f32; 3],
        dpdy: [f32; 3],
    ) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.cube.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], {%8, %9, %10, %10}, {%11, %12, %13, %13};", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, in("f") z, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdx[2], in("f") dpdy[0], in("f") dpdy[1], in("f") dpdy[2], options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.acube.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex_cube_layered(t: u64, x: f32, y: f32, z: f32, layer: u32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.acube.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %8}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") z, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.acube.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex_cube_layered_lod(
        t: u64,
        x: f32,
        y: f32,
        z: f32,
        layer: u32,
        level: f32,
    ) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.acube.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %8}], %9;", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") z, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.acube.v4.s32.f32`.
    #[inline(always)]
    unsafe fn tex_cube_layered_grad(
        t: u64,
        x: f32,
        y: f32,
        z: f32,
        layer: u32,
        dpdx: [f32; 3],
        dpdy: [f32; 3],
    ) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.acube.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %8}], {%9, %10, %11, %11}, {%12, %13, %14, %14};", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") z, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdx[2], in("f") dpdy[0], in("f") dpdy[1], in("f") dpdy[2], options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.2d.v4.s32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_sparse(t: u64, x: f32, y: f32) -> ([i32; 4], bool) {
        let (a, b, c, d): (i32, i32, i32, i32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.2d.v4.s32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.3d.v4.s32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex3d_sparse(t: u64, x: f32, y: f32, z: f32) -> ([i32; 4], bool) {
        let (a, b, c, d): (i32, i32, i32, i32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.3d.v4.s32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, in("f") z, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.a2d.v4.s32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_layered_sparse(t: u64, x: f32, y: f32, layer: u32) -> ([i32; 4], bool) {
        let (a, b, c, d): (i32, i32, i32, i32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.a2d.v4.s32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("r") layer, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.level.2d.v4.s32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_lod_sparse(t: u64, x: f32, y: f32, level: f32) -> ([i32; 4], bool) {
        let (a, b, c, d): (i32, i32, i32, i32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.level.2d.v4.s32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}], %8;\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, in("f") level, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.level.3d.v4.s32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex3d_lod_sparse(t: u64, x: f32, y: f32, z: f32, level: f32) -> ([i32; 4], bool) {
        let (a, b, c, d): (i32, i32, i32, i32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.level.3d.v4.s32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}], %9;\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, in("f") z, in("f") level, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.level.a2d.v4.s32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_layered_lod_sparse(
        t: u64,
        x: f32,
        y: f32,
        layer: u32,
        level: f32,
    ) -> ([i32; 4], bool) {
        let (a, b, c, d): (i32, i32, i32, i32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.level.a2d.v4.s32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}], %9;\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") level, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.grad.2d.v4.s32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_grad_sparse(
        t: u64,
        x: f32,
        y: f32,
        dpdx: [f32; 2],
        dpdy: [f32; 2],
    ) -> ([i32; 4], bool) {
        let (a, b, c, d): (i32, i32, i32, i32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.grad.2d.v4.s32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}], {%8, %9}, {%10, %11};\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdy[0], in("f") dpdy[1], options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.grad.3d.v4.s32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex3d_grad_sparse(
        t: u64,
        x: f32,
        y: f32,
        z: f32,
        dpdx: [f32; 3],
        dpdy: [f32; 3],
    ) -> ([i32; 4], bool) {
        let (a, b, c, d): (i32, i32, i32, i32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.grad.3d.v4.s32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}], {%9, %10, %11, %11}, {%12, %13, %14, %14};\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, in("f") z, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdx[2], in("f") dpdy[0], in("f") dpdy[1], in("f") dpdy[2], options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.grad.a2d.v4.s32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_layered_grad_sparse(
        t: u64,
        x: f32,
        y: f32,
        layer: u32,
        dpdx: [f32; 2],
        dpdy: [f32; 2],
    ) -> ([i32; 4], bool) {
        let (a, b, c, d): (i32, i32, i32, i32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.grad.a2d.v4.s32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}], {%9, %10}, {%11, %12};\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdy[0], in("f") dpdy[1], options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tld4.r.2d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn gather_r(t: u64, x: f32, y: f32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tld4.r.2d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tld4.r.2d.v4.s32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn gather_r_sparse(t: u64, x: f32, y: f32) -> ([i32; 4], bool) {
        let (a, b, c, d): (i32, i32, i32, i32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttld4.r.2d.v4.s32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tld4.g.2d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn gather_g(t: u64, x: f32, y: f32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tld4.g.2d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tld4.g.2d.v4.s32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn gather_g_sparse(t: u64, x: f32, y: f32) -> ([i32; 4], bool) {
        let (a, b, c, d): (i32, i32, i32, i32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttld4.g.2d.v4.s32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tld4.b.2d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn gather_b(t: u64, x: f32, y: f32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tld4.b.2d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tld4.b.2d.v4.s32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn gather_b_sparse(t: u64, x: f32, y: f32) -> ([i32; 4], bool) {
        let (a, b, c, d): (i32, i32, i32, i32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttld4.b.2d.v4.s32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tld4.a.2d.v4.s32.f32`.
    #[inline(always)]
    unsafe fn gather_a(t: u64, x: f32, y: f32) -> [i32; 4] {
        let (a, b, c, d): (i32, i32, i32, i32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tld4.a.2d.v4.s32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tld4.a.2d.v4.s32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn gather_a_sparse(t: u64, x: f32, y: f32) -> ([i32; 4], bool) {
        let (a, b, c, d): (i32, i32, i32, i32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttld4.a.2d.v4.s32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
}

impl sealed::Scalar for u32 {
    /// PTX `tex.1d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex1d(t: u64, x: f32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.1d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.1d.v4.u32.s32`.
    #[inline(always)]
    unsafe fn tex1d_fetch(t: u64, x: i32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.1d.v4.u32.s32 {%0, %1, %2, %3}, [%4, {%5}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") x, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.1d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex1d_lod(t: u64, x: f32, level: f32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.1d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5}], %6;", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.1d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex1d_grad(t: u64, x: f32, dpdx: f32, dpdy: f32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.1d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5}], {%6}, {%7};", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") dpdx, in("f") dpdy, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.a1d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex1d_layered(t: u64, x: f32, layer: u32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.a1d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.a1d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex1d_layered_lod(t: u64, x: f32, layer: u32, level: f32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.a1d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}], %7;", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.a1d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex1d_layered_grad(t: u64, x: f32, layer: u32, dpdx: f32, dpdy: f32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.a1d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}], {%7}, {%8};", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, in("f") dpdx, in("f") dpdy, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.2d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex2d(t: u64, x: f32, y: f32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.2d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.2d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex2d_lod(t: u64, x: f32, y: f32, level: f32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.2d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}], %7;", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.2d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex2d_grad(t: u64, x: f32, y: f32, dpdx: [f32; 2], dpdy: [f32; 2]) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.2d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}], {%7, %8}, {%9, %10};", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdy[0], in("f") dpdy[1], options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.a2d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex2d_layered(t: u64, x: f32, y: f32, layer: u32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.a2d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.a2d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex2d_layered_lod(t: u64, x: f32, y: f32, layer: u32, level: f32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.a2d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], %8;", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.a2d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex2d_layered_grad(
        t: u64,
        x: f32,
        y: f32,
        layer: u32,
        dpdx: [f32; 2],
        dpdy: [f32; 2],
    ) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.a2d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], {%8, %9}, {%10, %11};", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdy[0], in("f") dpdy[1], options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.3d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex3d(t: u64, x: f32, y: f32, z: f32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.3d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, in("f") z, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.3d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex3d_lod(t: u64, x: f32, y: f32, z: f32, level: f32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.3d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], %8;", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, in("f") z, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.3d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex3d_grad(
        t: u64,
        x: f32,
        y: f32,
        z: f32,
        dpdx: [f32; 3],
        dpdy: [f32; 3],
    ) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.3d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], {%8, %9, %10, %10}, {%11, %12, %13, %13};", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, in("f") z, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdx[2], in("f") dpdy[0], in("f") dpdy[1], in("f") dpdy[2], options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.cube.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex_cube(t: u64, x: f32, y: f32, z: f32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.cube.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, in("f") z, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.cube.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex_cube_lod(t: u64, x: f32, y: f32, z: f32, level: f32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.cube.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], %8;", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, in("f") z, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.cube.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex_cube_grad(
        t: u64,
        x: f32,
        y: f32,
        z: f32,
        dpdx: [f32; 3],
        dpdy: [f32; 3],
    ) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.cube.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}], {%8, %9, %10, %10}, {%11, %12, %13, %13};", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, in("f") z, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdx[2], in("f") dpdy[0], in("f") dpdy[1], in("f") dpdy[2], options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.acube.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex_cube_layered(t: u64, x: f32, y: f32, z: f32, layer: u32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.acube.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %8}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") z, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.level.acube.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex_cube_layered_lod(
        t: u64,
        x: f32,
        y: f32,
        z: f32,
        layer: u32,
        level: f32,
    ) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.level.acube.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %8}], %9;", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") z, in("f") level, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.grad.acube.v4.u32.f32`.
    #[inline(always)]
    unsafe fn tex_cube_layered_grad(
        t: u64,
        x: f32,
        y: f32,
        z: f32,
        layer: u32,
        dpdx: [f32; 3],
        dpdy: [f32; 3],
    ) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tex.grad.acube.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6, %7, %8}], {%9, %10, %11, %11}, {%12, %13, %14, %14};", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") z, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdx[2], in("f") dpdy[0], in("f") dpdy[1], in("f") dpdy[2], options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tex.2d.v4.u32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_sparse(t: u64, x: f32, y: f32) -> ([u32; 4], bool) {
        let (a, b, c, d): (u32, u32, u32, u32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.2d.v4.u32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.3d.v4.u32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex3d_sparse(t: u64, x: f32, y: f32, z: f32) -> ([u32; 4], bool) {
        let (a, b, c, d): (u32, u32, u32, u32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.3d.v4.u32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, in("f") z, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.a2d.v4.u32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_layered_sparse(t: u64, x: f32, y: f32, layer: u32) -> ([u32; 4], bool) {
        let (a, b, c, d): (u32, u32, u32, u32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.a2d.v4.u32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("r") layer, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.level.2d.v4.u32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_lod_sparse(t: u64, x: f32, y: f32, level: f32) -> ([u32; 4], bool) {
        let (a, b, c, d): (u32, u32, u32, u32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.level.2d.v4.u32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}], %8;\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, in("f") level, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.level.3d.v4.u32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex3d_lod_sparse(t: u64, x: f32, y: f32, z: f32, level: f32) -> ([u32; 4], bool) {
        let (a, b, c, d): (u32, u32, u32, u32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.level.3d.v4.u32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}], %9;\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, in("f") z, in("f") level, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.level.a2d.v4.u32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_layered_lod_sparse(
        t: u64,
        x: f32,
        y: f32,
        layer: u32,
        level: f32,
    ) -> ([u32; 4], bool) {
        let (a, b, c, d): (u32, u32, u32, u32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.level.a2d.v4.u32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}], %9;\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") level, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.grad.2d.v4.u32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_grad_sparse(
        t: u64,
        x: f32,
        y: f32,
        dpdx: [f32; 2],
        dpdy: [f32; 2],
    ) -> ([u32; 4], bool) {
        let (a, b, c, d): (u32, u32, u32, u32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.grad.2d.v4.u32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}], {%8, %9}, {%10, %11};\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdy[0], in("f") dpdy[1], options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.grad.3d.v4.u32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex3d_grad_sparse(
        t: u64,
        x: f32,
        y: f32,
        z: f32,
        dpdx: [f32; 3],
        dpdy: [f32; 3],
    ) -> ([u32; 4], bool) {
        let (a, b, c, d): (u32, u32, u32, u32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.grad.3d.v4.u32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}], {%9, %10, %11, %11}, {%12, %13, %14, %14};\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, in("f") z, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdx[2], in("f") dpdy[0], in("f") dpdy[1], in("f") dpdy[2], options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tex.grad.a2d.v4.u32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn tex2d_layered_grad_sparse(
        t: u64,
        x: f32,
        y: f32,
        layer: u32,
        dpdx: [f32; 2],
        dpdy: [f32; 2],
    ) -> ([u32; 4], bool) {
        let (a, b, c, d): (u32, u32, u32, u32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttex.grad.a2d.v4.u32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7, %8, %8}], {%9, %10}, {%11, %12};\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("r") layer, in("f") x, in("f") y, in("f") dpdx[0], in("f") dpdx[1], in("f") dpdy[0], in("f") dpdy[1], options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tld4.r.2d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn gather_r(t: u64, x: f32, y: f32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tld4.r.2d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tld4.r.2d.v4.u32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn gather_r_sparse(t: u64, x: f32, y: f32) -> ([u32; 4], bool) {
        let (a, b, c, d): (u32, u32, u32, u32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttld4.r.2d.v4.u32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tld4.g.2d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn gather_g(t: u64, x: f32, y: f32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tld4.g.2d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tld4.g.2d.v4.u32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn gather_g_sparse(t: u64, x: f32, y: f32) -> ([u32; 4], bool) {
        let (a, b, c, d): (u32, u32, u32, u32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttld4.g.2d.v4.u32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tld4.b.2d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn gather_b(t: u64, x: f32, y: f32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tld4.b.2d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tld4.b.2d.v4.u32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn gather_b_sparse(t: u64, x: f32, y: f32) -> ([u32; 4], bool) {
        let (a, b, c, d): (u32, u32, u32, u32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttld4.b.2d.v4.u32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
    /// PTX `tld4.a.2d.v4.u32.f32`.
    #[inline(always)]
    unsafe fn gather_a(t: u64, x: f32, y: f32) -> [u32; 4] {
        let (a, b, c, d): (u32, u32, u32, u32);
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("tld4.a.2d.v4.u32.f32 {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        [a, b, c, d]
    }
    /// PTX `tld4.a.2d.v4.u32.f32` with its residency predicate (`selp.u16`).
    #[inline(always)]
    unsafe fn gather_a_sparse(t: u64, x: f32, y: f32) -> ([u32; 4], bool) {
        let (a, b, c, d): (u32, u32, u32, u32);
        let r: u16;
        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.
        unsafe {
            ptx_asm!("{.reg .pred %%p0;\n\ttld4.a.2d.v4.u32.f32 {%0, %1, %2, %3}|%%p0, [%5, {%6, %7}];\n\tselp.u16 %4, 1, 0, %%p0; }", out("=r") a, out("=r") b, out("=r") c, out("=r") d, out("=h") r, in("l") t, in("f") x, in("f") y, options(register_only))
        };
        ([a, b, c, d], r != 0)
    }
}

/// `tex1D<T>`: PTX `tex.1d.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a 1D resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex1d<T: TexElem>(tex: TexObject, x: f32) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex1d(tex, x) })
}

/// `tex1Dfetch<T>`: PTX `tex.1d.v4.{f32,s32,u32}.s32`, keeping the first channels `T` holds.
///
/// `x` is an element index; the fetch is unfiltered and unnormalized.
///
/// # Safety
///
/// `tex` must be a live texture object over a linear-memory (`CU_RESOURCE_TYPE_LINEAR`) resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex1d_fetch<T: TexElem>(tex: TexObject, x: i32) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex1d_fetch(tex, x) })
}

/// `tex1DLod<T>`: PTX `tex.level.1d.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a 1D (mipmapped) resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex1d_lod<T: TexElem>(tex: TexObject, x: f32, level: f32) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex1d_lod(tex, x, level) })
}

/// `tex1DGrad<T>`: PTX `tex.grad.1d.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a 1D (mipmapped) resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex1d_grad<T: TexElem>(tex: TexObject, x: f32, dpdx: f32, dpdy: f32) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex1d_grad(tex, x, dpdx, dpdy) })
}

/// `tex1DLayered<T>`: PTX `tex.a1d.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a 1D layered resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex1d_layered<T: TexElem>(tex: TexObject, x: f32, layer: u32) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex1d_layered(tex, x, layer) })
}

/// `tex1DLayeredLod<T>`: PTX `tex.level.a1d.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a 1D layered (mipmapped) resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex1d_layered_lod<T: TexElem>(tex: TexObject, x: f32, layer: u32, level: f32) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex1d_layered_lod(tex, x, layer, level) })
}

/// `tex1DLayeredGrad<T>`: PTX `tex.grad.a1d.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a 1D layered (mipmapped) resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex1d_layered_grad<T: TexElem>(
    tex: TexObject,
    x: f32,
    layer: u32,
    dpdx: f32,
    dpdy: f32,
) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex1d_layered_grad(tex, x, layer, dpdx, dpdy) })
}

/// `tex2D<T>`: PTX `tex.2d.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a 2D (array or pitched) resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex2d<T: TexElem>(tex: TexObject, x: f32, y: f32) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex2d(tex, x, y) })
}

/// `tex2DLod<T>`: PTX `tex.level.2d.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a 2D (mipmapped) resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex2d_lod<T: TexElem>(tex: TexObject, x: f32, y: f32, level: f32) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex2d_lod(tex, x, y, level) })
}

/// `tex2DGrad<T>`: PTX `tex.grad.2d.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a 2D (mipmapped) resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex2d_grad<T: TexElem>(
    tex: TexObject,
    x: f32,
    y: f32,
    dpdx: [f32; 2],
    dpdy: [f32; 2],
) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex2d_grad(tex, x, y, dpdx, dpdy) })
}

/// `tex2DLayered<T>`: PTX `tex.a2d.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a 2D layered resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex2d_layered<T: TexElem>(tex: TexObject, x: f32, y: f32, layer: u32) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex2d_layered(tex, x, y, layer) })
}

/// `tex2DLayeredLod<T>`: PTX `tex.level.a2d.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a 2D layered (mipmapped) resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex2d_layered_lod<T: TexElem>(
    tex: TexObject,
    x: f32,
    y: f32,
    layer: u32,
    level: f32,
) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex2d_layered_lod(tex, x, y, layer, level) })
}

/// `tex2DLayeredGrad<T>`: PTX `tex.grad.a2d.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a 2D layered (mipmapped) resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex2d_layered_grad<T: TexElem>(
    tex: TexObject,
    x: f32,
    y: f32,
    layer: u32,
    dpdx: [f32; 2],
    dpdy: [f32; 2],
) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex2d_layered_grad(tex, x, y, layer, dpdx, dpdy) })
}

/// `tex3D<T>`: PTX `tex.3d.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a 3D resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex3d<T: TexElem>(tex: TexObject, x: f32, y: f32, z: f32) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex3d(tex, x, y, z) })
}

/// `tex3DLod<T>`: PTX `tex.level.3d.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a 3D (mipmapped) resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex3d_lod<T: TexElem>(tex: TexObject, x: f32, y: f32, z: f32, level: f32) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex3d_lod(tex, x, y, z, level) })
}

/// `tex3DGrad<T>`: PTX `tex.grad.3d.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a 3D (mipmapped) resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex3d_grad<T: TexElem>(
    tex: TexObject,
    x: f32,
    y: f32,
    z: f32,
    dpdx: [f32; 3],
    dpdy: [f32; 3],
) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex3d_grad(tex, x, y, z, dpdx, dpdy) })
}

/// `texCubemap<T>`: PTX `tex.cube.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a cubemap (`(x, y, z)` is a direction) resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex_cube<T: TexElem>(tex: TexObject, x: f32, y: f32, z: f32) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex_cube(tex, x, y, z) })
}

/// `texCubemapLod<T>`: PTX `tex.level.cube.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a cubemap (mipmapped) resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex_cube_lod<T: TexElem>(tex: TexObject, x: f32, y: f32, z: f32, level: f32) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex_cube_lod(tex, x, y, z, level) })
}

/// `texCubemapGrad<T>`: PTX `tex.grad.cube.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a cubemap (mipmapped) resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex_cube_grad<T: TexElem>(
    tex: TexObject,
    x: f32,
    y: f32,
    z: f32,
    dpdx: [f32; 3],
    dpdy: [f32; 3],
) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex_cube_grad(tex, x, y, z, dpdx, dpdy) })
}

/// `texCubemapLayered<T>`: PTX `tex.acube.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a cubemap layered resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex_cube_layered<T: TexElem>(
    tex: TexObject,
    x: f32,
    y: f32,
    z: f32,
    layer: u32,
) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex_cube_layered(tex, x, y, z, layer) })
}

/// `texCubemapLayeredLod<T>`: PTX `tex.level.acube.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a cubemap layered (mipmapped) resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex_cube_layered_lod<T: TexElem>(
    tex: TexObject,
    x: f32,
    y: f32,
    z: f32,
    layer: u32,
    level: f32,
) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex_cube_layered_lod(tex, x, y, z, layer, level) })
}

/// `texCubemapLayeredGrad<T>`: PTX `tex.grad.acube.v4.{f32,s32,u32}.f32`, keeping the first channels `T` holds.
///
/// # Safety
///
/// `tex` must be a live texture object over a cubemap layered (mipmapped) resource.
/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or
/// OptiX program.
#[inline(always)]
pub unsafe fn tex_cube_layered_grad<T: TexElem>(
    tex: TexObject,
    x: f32,
    y: f32,
    z: f32,
    layer: u32,
    dpdx: [f32; 3],
    dpdy: [f32; 3],
) -> T {
    // SAFETY: forwarded contract.
    T::from_v4(unsafe { <T::S as Scalar>::tex_cube_layered_grad(tex, x, y, z, layer, dpdx, dpdy) })
}

/// `tex2D<T>(..., bool* isResident)`: [`tex2d`] plus whether every texel the fetch touches is
/// resident (PTX `tex.2d.v4.{f32,s32,u32}.f32` with a `|p` predicate destination).
///
/// # Safety
///
/// As [`tex2d`]; sm_60 or later (CUDA's `__NV_TEX_SPARSE`).
#[inline(always)]
pub unsafe fn tex2d_sparse<T: TexElem>(tex: TexObject, x: f32, y: f32) -> (T, bool) {
    // SAFETY: forwarded contract.
    let (v, r) = unsafe { <T::S as Scalar>::tex2d_sparse(tex, x, y) };
    (T::from_v4(v), r)
}

/// `tex3D<T>(..., bool* isResident)`: [`tex3d`] plus whether every texel the fetch touches is
/// resident (PTX `tex.3d.v4.{f32,s32,u32}.f32` with a `|p` predicate destination).
///
/// # Safety
///
/// As [`tex3d`]; sm_60 or later (CUDA's `__NV_TEX_SPARSE`).
#[inline(always)]
pub unsafe fn tex3d_sparse<T: TexElem>(tex: TexObject, x: f32, y: f32, z: f32) -> (T, bool) {
    // SAFETY: forwarded contract.
    let (v, r) = unsafe { <T::S as Scalar>::tex3d_sparse(tex, x, y, z) };
    (T::from_v4(v), r)
}

/// `tex2DLayered<T>(..., bool* isResident)`: [`tex2d_layered`] plus whether every texel the fetch touches is
/// resident (PTX `tex.a2d.v4.{f32,s32,u32}.f32` with a `|p` predicate destination).
///
/// # Safety
///
/// As [`tex2d_layered`]; sm_60 or later (CUDA's `__NV_TEX_SPARSE`).
#[inline(always)]
pub unsafe fn tex2d_layered_sparse<T: TexElem>(
    tex: TexObject,
    x: f32,
    y: f32,
    layer: u32,
) -> (T, bool) {
    // SAFETY: forwarded contract.
    let (v, r) = unsafe { <T::S as Scalar>::tex2d_layered_sparse(tex, x, y, layer) };
    (T::from_v4(v), r)
}

/// `tex2DLod<T>(..., bool* isResident)`: [`tex2d_lod`] plus whether every texel the fetch touches is
/// resident (PTX `tex.level.2d.v4.{f32,s32,u32}.f32` with a `|p` predicate destination).
///
/// # Safety
///
/// As [`tex2d_lod`]; sm_60 or later (CUDA's `__NV_TEX_SPARSE`).
#[inline(always)]
pub unsafe fn tex2d_lod_sparse<T: TexElem>(
    tex: TexObject,
    x: f32,
    y: f32,
    level: f32,
) -> (T, bool) {
    // SAFETY: forwarded contract.
    let (v, r) = unsafe { <T::S as Scalar>::tex2d_lod_sparse(tex, x, y, level) };
    (T::from_v4(v), r)
}

/// `tex3DLod<T>(..., bool* isResident)`: [`tex3d_lod`] plus whether every texel the fetch touches is
/// resident (PTX `tex.level.3d.v4.{f32,s32,u32}.f32` with a `|p` predicate destination).
///
/// # Safety
///
/// As [`tex3d_lod`]; sm_60 or later (CUDA's `__NV_TEX_SPARSE`).
#[inline(always)]
pub unsafe fn tex3d_lod_sparse<T: TexElem>(
    tex: TexObject,
    x: f32,
    y: f32,
    z: f32,
    level: f32,
) -> (T, bool) {
    // SAFETY: forwarded contract.
    let (v, r) = unsafe { <T::S as Scalar>::tex3d_lod_sparse(tex, x, y, z, level) };
    (T::from_v4(v), r)
}

/// `tex2DLayeredLod<T>(..., bool* isResident)`: [`tex2d_layered_lod`] plus whether every texel the fetch touches is
/// resident (PTX `tex.level.a2d.v4.{f32,s32,u32}.f32` with a `|p` predicate destination).
///
/// # Safety
///
/// As [`tex2d_layered_lod`]; sm_60 or later (CUDA's `__NV_TEX_SPARSE`).
#[inline(always)]
pub unsafe fn tex2d_layered_lod_sparse<T: TexElem>(
    tex: TexObject,
    x: f32,
    y: f32,
    layer: u32,
    level: f32,
) -> (T, bool) {
    // SAFETY: forwarded contract.
    let (v, r) = unsafe { <T::S as Scalar>::tex2d_layered_lod_sparse(tex, x, y, layer, level) };
    (T::from_v4(v), r)
}

/// `tex2DGrad<T>(..., bool* isResident)`: [`tex2d_grad`] plus whether every texel the fetch touches is
/// resident (PTX `tex.grad.2d.v4.{f32,s32,u32}.f32` with a `|p` predicate destination).
///
/// # Safety
///
/// As [`tex2d_grad`]; sm_60 or later (CUDA's `__NV_TEX_SPARSE`).
#[inline(always)]
pub unsafe fn tex2d_grad_sparse<T: TexElem>(
    tex: TexObject,
    x: f32,
    y: f32,
    dpdx: [f32; 2],
    dpdy: [f32; 2],
) -> (T, bool) {
    // SAFETY: forwarded contract.
    let (v, r) = unsafe { <T::S as Scalar>::tex2d_grad_sparse(tex, x, y, dpdx, dpdy) };
    (T::from_v4(v), r)
}

/// `tex3DGrad<T>(..., bool* isResident)`: [`tex3d_grad`] plus whether every texel the fetch touches is
/// resident (PTX `tex.grad.3d.v4.{f32,s32,u32}.f32` with a `|p` predicate destination).
///
/// # Safety
///
/// As [`tex3d_grad`]; sm_60 or later (CUDA's `__NV_TEX_SPARSE`).
#[inline(always)]
pub unsafe fn tex3d_grad_sparse<T: TexElem>(
    tex: TexObject,
    x: f32,
    y: f32,
    z: f32,
    dpdx: [f32; 3],
    dpdy: [f32; 3],
) -> (T, bool) {
    // SAFETY: forwarded contract.
    let (v, r) = unsafe { <T::S as Scalar>::tex3d_grad_sparse(tex, x, y, z, dpdx, dpdy) };
    (T::from_v4(v), r)
}

/// `tex2DLayeredGrad<T>(..., bool* isResident)`: [`tex2d_layered_grad`] plus whether every texel the fetch touches is
/// resident (PTX `tex.grad.a2d.v4.{f32,s32,u32}.f32` with a `|p` predicate destination).
///
/// # Safety
///
/// As [`tex2d_layered_grad`]; sm_60 or later (CUDA's `__NV_TEX_SPARSE`).
#[inline(always)]
pub unsafe fn tex2d_layered_grad_sparse<T: TexElem>(
    tex: TexObject,
    x: f32,
    y: f32,
    layer: u32,
    dpdx: [f32; 2],
    dpdy: [f32; 2],
) -> (T, bool) {
    // SAFETY: forwarded contract.
    let (v, r) =
        unsafe { <T::S as Scalar>::tex2d_layered_grad_sparse(tex, x, y, layer, dpdx, dpdy) };
    (T::from_v4(v), r)
}

/// `tex2Dgather<T>(tex, x, y, COMP)`: PTX `tld4.{r,g,b,a}.2d.v4.{f32,s32,u32}.f32` — channel
/// `COMP` (0 = r, 1 = g, 2 = b, 3 = a) of the four texels bilinear filtering at `(x, y)` would
/// use, in the order of CUDA's `tex2Dgather`.
///
/// # Safety
///
/// `tex` must be a live texture object over a 2D resource; [`TexElem`] says which `T` matches
/// its read mode. Valid in any kernel, device function or OptiX program.
#[inline(always)]
pub unsafe fn tex2d_gather<T: TexElem, const COMP: u32>(tex: TexObject, x: f32, y: f32) -> T {
    const { assert!(COMP < 4, "tex2d_gather: COMP selects r, g, b or a (0..=3)") };
    // SAFETY: forwarded contract; `COMP` is a constant, so one arm survives.
    T::from_v4(unsafe {
        match COMP {
            0 => <T::S as Scalar>::gather_r(tex, x, y),
            1 => <T::S as Scalar>::gather_g(tex, x, y),
            2 => <T::S as Scalar>::gather_b(tex, x, y),
            _ => <T::S as Scalar>::gather_a(tex, x, y),
        }
    })
}

/// `tex2Dgather<T>(tex, x, y, &isResident, COMP)`: [`tex2d_gather`] plus whether every texel
/// is resident (PTX `tld4.{r,g,b,a}.2d.v4.{f32,s32,u32}.f32` with a `|p` predicate destination).
///
/// # Safety
///
/// As [`tex2d_gather`]; sm_60 or later (CUDA's `__NV_TEX_SPARSE`).
#[inline(always)]
pub unsafe fn tex2d_gather_sparse<T: TexElem, const COMP: u32>(
    tex: TexObject,
    x: f32,
    y: f32,
) -> (T, bool) {
    const {
        assert!(
            COMP < 4,
            "tex2d_gather_sparse: COMP selects r, g, b or a (0..=3)"
        )
    };
    // SAFETY: forwarded contract; `COMP` is a constant, so one arm survives.
    let (v, r) = unsafe {
        match COMP {
            0 => <T::S as Scalar>::gather_r_sparse(tex, x, y),
            1 => <T::S as Scalar>::gather_g_sparse(tex, x, y),
            2 => <T::S as Scalar>::gather_b_sparse(tex, x, y),
            _ => <T::S as Scalar>::gather_a_sparse(tex, x, y),
        }
    };
    (T::from_v4(v), r)
}

/// PTX `suld.b.1d.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_1d_b8_1(s: u64, x: i32, m: Boundary) -> [u16; 1] {
    let a: u16;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.1d.b8.trap {%0}, [%1, {%2}];", out("=h") a, in("l") s, in("r") x)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.1d.b8.clamp {%0}, [%1, {%2}];", out("=h") a, in("l") s, in("r") x)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.1d.b8.zero {%0}, [%1, {%2}];", out("=h") a, in("l") s, in("r") x)
            }
        }
    }
    [a]
}

/// PTX `sust.b.1d.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_1d_b8_1(v: [u16; 1], s: u64, x: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.1d.b8.trap [%0, {%1}], {%2};", in("l") s, in("r") x, in("h") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.1d.b8.clamp [%0, {%1}], {%2};", in("l") s, in("r") x, in("h") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.1d.b8.zero [%0, {%1}], {%2};", in("l") s, in("r") x, in("h") v[0])
            }
        }
    }
}

/// PTX `suld.b.1d.v2.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_1d_b8_2(s: u64, x: i32, m: Boundary) -> [u16; 2] {
    let (a, b): (u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.1d.v2.b8.trap {%0, %1}, [%2, {%3}];", out("=h") a, out("=h") b, in("l") s, in("r") x)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.1d.v2.b8.clamp {%0, %1}, [%2, {%3}];", out("=h") a, out("=h") b, in("l") s, in("r") x)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.1d.v2.b8.zero {%0, %1}, [%2, {%3}];", out("=h") a, out("=h") b, in("l") s, in("r") x)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.1d.v2.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_1d_b8_2(v: [u16; 2], s: u64, x: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.1d.v2.b8.trap [%0, {%1}], {%2, %3};", in("l") s, in("r") x, in("h") v[0], in("h") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.1d.v2.b8.clamp [%0, {%1}], {%2, %3};", in("l") s, in("r") x, in("h") v[0], in("h") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.1d.v2.b8.zero [%0, {%1}], {%2, %3};", in("l") s, in("r") x, in("h") v[0], in("h") v[1])
            }
        }
    }
}

/// PTX `suld.b.1d.v4.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_1d_b8_4(s: u64, x: i32, m: Boundary) -> [u16; 4] {
    let (a, b, c, d): (u16, u16, u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.1d.v4.b8.trap {%0, %1, %2, %3}, [%4, {%5}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.1d.v4.b8.clamp {%0, %1, %2, %3}, [%4, {%5}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.1d.v4.b8.zero {%0, %1, %2, %3}, [%4, {%5}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x)
            }
        }
    }
    [a, b, c, d]
}

/// PTX `sust.b.1d.v4.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_1d_b8_4(v: [u16; 4], s: u64, x: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.1d.v4.b8.trap [%0, {%1}], {%2, %3, %4, %5};", in("l") s, in("r") x, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.1d.v4.b8.clamp [%0, {%1}], {%2, %3, %4, %5};", in("l") s, in("r") x, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.1d.v4.b8.zero [%0, {%1}], {%2, %3, %4, %5};", in("l") s, in("r") x, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
        }
    }
}

/// PTX `suld.b.1d.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_1d_b16_1(s: u64, x: i32, m: Boundary) -> [u16; 1] {
    let a: u16;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.1d.b16.trap {%0}, [%1, {%2}];", out("=h") a, in("l") s, in("r") x)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.1d.b16.clamp {%0}, [%1, {%2}];", out("=h") a, in("l") s, in("r") x)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.1d.b16.zero {%0}, [%1, {%2}];", out("=h") a, in("l") s, in("r") x)
            }
        }
    }
    [a]
}

/// PTX `sust.b.1d.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_1d_b16_1(v: [u16; 1], s: u64, x: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.1d.b16.trap [%0, {%1}], {%2};", in("l") s, in("r") x, in("h") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.1d.b16.clamp [%0, {%1}], {%2};", in("l") s, in("r") x, in("h") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.1d.b16.zero [%0, {%1}], {%2};", in("l") s, in("r") x, in("h") v[0])
            }
        }
    }
}

/// PTX `suld.b.1d.v2.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_1d_b16_2(s: u64, x: i32, m: Boundary) -> [u16; 2] {
    let (a, b): (u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.1d.v2.b16.trap {%0, %1}, [%2, {%3}];", out("=h") a, out("=h") b, in("l") s, in("r") x)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.1d.v2.b16.clamp {%0, %1}, [%2, {%3}];", out("=h") a, out("=h") b, in("l") s, in("r") x)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.1d.v2.b16.zero {%0, %1}, [%2, {%3}];", out("=h") a, out("=h") b, in("l") s, in("r") x)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.1d.v2.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_1d_b16_2(v: [u16; 2], s: u64, x: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.1d.v2.b16.trap [%0, {%1}], {%2, %3};", in("l") s, in("r") x, in("h") v[0], in("h") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.1d.v2.b16.clamp [%0, {%1}], {%2, %3};", in("l") s, in("r") x, in("h") v[0], in("h") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.1d.v2.b16.zero [%0, {%1}], {%2, %3};", in("l") s, in("r") x, in("h") v[0], in("h") v[1])
            }
        }
    }
}

/// PTX `suld.b.1d.v4.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_1d_b16_4(s: u64, x: i32, m: Boundary) -> [u16; 4] {
    let (a, b, c, d): (u16, u16, u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.1d.v4.b16.trap {%0, %1, %2, %3}, [%4, {%5}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.1d.v4.b16.clamp {%0, %1, %2, %3}, [%4, {%5}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.1d.v4.b16.zero {%0, %1, %2, %3}, [%4, {%5}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x)
            }
        }
    }
    [a, b, c, d]
}

/// PTX `sust.b.1d.v4.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_1d_b16_4(v: [u16; 4], s: u64, x: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.1d.v4.b16.trap [%0, {%1}], {%2, %3, %4, %5};", in("l") s, in("r") x, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.1d.v4.b16.clamp [%0, {%1}], {%2, %3, %4, %5};", in("l") s, in("r") x, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.1d.v4.b16.zero [%0, {%1}], {%2, %3, %4, %5};", in("l") s, in("r") x, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
        }
    }
}

/// PTX `suld.b.1d.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_1d_b32_1(s: u64, x: i32, m: Boundary) -> [u32; 1] {
    let a: u32;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.1d.b32.trap {%0}, [%1, {%2}];", out("=r") a, in("l") s, in("r") x)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.1d.b32.clamp {%0}, [%1, {%2}];", out("=r") a, in("l") s, in("r") x)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.1d.b32.zero {%0}, [%1, {%2}];", out("=r") a, in("l") s, in("r") x)
            }
        }
    }
    [a]
}

/// PTX `sust.b.1d.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_1d_b32_1(v: [u32; 1], s: u64, x: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.1d.b32.trap [%0, {%1}], {%2};", in("l") s, in("r") x, in("r") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.1d.b32.clamp [%0, {%1}], {%2};", in("l") s, in("r") x, in("r") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.1d.b32.zero [%0, {%1}], {%2};", in("l") s, in("r") x, in("r") v[0])
            }
        }
    }
}

/// PTX `suld.b.1d.v2.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_1d_b32_2(s: u64, x: i32, m: Boundary) -> [u32; 2] {
    let (a, b): (u32, u32);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.1d.v2.b32.trap {%0, %1}, [%2, {%3}];", out("=r") a, out("=r") b, in("l") s, in("r") x)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.1d.v2.b32.clamp {%0, %1}, [%2, {%3}];", out("=r") a, out("=r") b, in("l") s, in("r") x)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.1d.v2.b32.zero {%0, %1}, [%2, {%3}];", out("=r") a, out("=r") b, in("l") s, in("r") x)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.1d.v2.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_1d_b32_2(v: [u32; 2], s: u64, x: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.1d.v2.b32.trap [%0, {%1}], {%2, %3};", in("l") s, in("r") x, in("r") v[0], in("r") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.1d.v2.b32.clamp [%0, {%1}], {%2, %3};", in("l") s, in("r") x, in("r") v[0], in("r") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.1d.v2.b32.zero [%0, {%1}], {%2, %3};", in("l") s, in("r") x, in("r") v[0], in("r") v[1])
            }
        }
    }
}

/// PTX `suld.b.1d.v4.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_1d_b32_4(s: u64, x: i32, m: Boundary) -> [u32; 4] {
    let (a, b, c, d): (u32, u32, u32, u32);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.1d.v4.b32.trap {%0, %1, %2, %3}, [%4, {%5}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") s, in("r") x)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.1d.v4.b32.clamp {%0, %1, %2, %3}, [%4, {%5}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") s, in("r") x)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.1d.v4.b32.zero {%0, %1, %2, %3}, [%4, {%5}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") s, in("r") x)
            }
        }
    }
    [a, b, c, d]
}

/// PTX `sust.b.1d.v4.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_1d_b32_4(v: [u32; 4], s: u64, x: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.1d.v4.b32.trap [%0, {%1}], {%2, %3, %4, %5};", in("l") s, in("r") x, in("r") v[0], in("r") v[1], in("r") v[2], in("r") v[3])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.1d.v4.b32.clamp [%0, {%1}], {%2, %3, %4, %5};", in("l") s, in("r") x, in("r") v[0], in("r") v[1], in("r") v[2], in("r") v[3])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.1d.v4.b32.zero [%0, {%1}], {%2, %3, %4, %5};", in("l") s, in("r") x, in("r") v[0], in("r") v[1], in("r") v[2], in("r") v[3])
            }
        }
    }
}

/// PTX `suld.b.1d.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_1d_b64_1(s: u64, x: i32, m: Boundary) -> [u64; 1] {
    let a: u64;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.1d.b64.trap {%0}, [%1, {%2}];", out("=l") a, in("l") s, in("r") x)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.1d.b64.clamp {%0}, [%1, {%2}];", out("=l") a, in("l") s, in("r") x)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.1d.b64.zero {%0}, [%1, {%2}];", out("=l") a, in("l") s, in("r") x)
            }
        }
    }
    [a]
}

/// PTX `sust.b.1d.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_1d_b64_1(v: [u64; 1], s: u64, x: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.1d.b64.trap [%0, {%1}], {%2};", in("l") s, in("r") x, in("l") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.1d.b64.clamp [%0, {%1}], {%2};", in("l") s, in("r") x, in("l") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.1d.b64.zero [%0, {%1}], {%2};", in("l") s, in("r") x, in("l") v[0])
            }
        }
    }
}

/// PTX `suld.b.1d.v2.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_1d_b64_2(s: u64, x: i32, m: Boundary) -> [u64; 2] {
    let (a, b): (u64, u64);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.1d.v2.b64.trap {%0, %1}, [%2, {%3}];", out("=l") a, out("=l") b, in("l") s, in("r") x)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.1d.v2.b64.clamp {%0, %1}, [%2, {%3}];", out("=l") a, out("=l") b, in("l") s, in("r") x)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.1d.v2.b64.zero {%0, %1}, [%2, {%3}];", out("=l") a, out("=l") b, in("l") s, in("r") x)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.1d.v2.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_1d_b64_2(v: [u64; 2], s: u64, x: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.1d.v2.b64.trap [%0, {%1}], {%2, %3};", in("l") s, in("r") x, in("l") v[0], in("l") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.1d.v2.b64.clamp [%0, {%1}], {%2, %3};", in("l") s, in("r") x, in("l") v[0], in("l") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.1d.v2.b64.zero [%0, {%1}], {%2, %3};", in("l") s, in("r") x, in("l") v[0], in("l") v[1])
            }
        }
    }
}

/// PTX `suld.b.2d.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_2d_b8_1(s: u64, x: i32, y: i32, m: Boundary) -> [u16; 1] {
    let a: u16;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.2d.b8.trap {%0}, [%1, {%2, %3}];", out("=h") a, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.2d.b8.clamp {%0}, [%1, {%2, %3}];", out("=h") a, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.2d.b8.zero {%0}, [%1, {%2, %3}];", out("=h") a, in("l") s, in("r") x, in("r") y)
            }
        }
    }
    [a]
}

/// PTX `sust.b.2d.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_2d_b8_1(v: [u16; 1], s: u64, x: i32, y: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.2d.b8.trap [%0, {%1, %2}], {%3};", in("l") s, in("r") x, in("r") y, in("h") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.2d.b8.clamp [%0, {%1, %2}], {%3};", in("l") s, in("r") x, in("r") y, in("h") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.2d.b8.zero [%0, {%1, %2}], {%3};", in("l") s, in("r") x, in("r") y, in("h") v[0])
            }
        }
    }
}

/// PTX `suld.b.2d.v2.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_2d_b8_2(s: u64, x: i32, y: i32, m: Boundary) -> [u16; 2] {
    let (a, b): (u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.2d.v2.b8.trap {%0, %1}, [%2, {%3, %4}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.2d.v2.b8.clamp {%0, %1}, [%2, {%3, %4}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.2d.v2.b8.zero {%0, %1}, [%2, {%3, %4}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.2d.v2.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_2d_b8_2(v: [u16; 2], s: u64, x: i32, y: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.2d.v2.b8.trap [%0, {%1, %2}], {%3, %4};", in("l") s, in("r") x, in("r") y, in("h") v[0], in("h") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.2d.v2.b8.clamp [%0, {%1, %2}], {%3, %4};", in("l") s, in("r") x, in("r") y, in("h") v[0], in("h") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.2d.v2.b8.zero [%0, {%1, %2}], {%3, %4};", in("l") s, in("r") x, in("r") y, in("h") v[0], in("h") v[1])
            }
        }
    }
}

/// PTX `suld.b.2d.v4.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_2d_b8_4(s: u64, x: i32, y: i32, m: Boundary) -> [u16; 4] {
    let (a, b, c, d): (u16, u16, u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.2d.v4.b8.trap {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.2d.v4.b8.clamp {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.2d.v4.b8.zero {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y)
            }
        }
    }
    [a, b, c, d]
}

/// PTX `sust.b.2d.v4.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_2d_b8_4(v: [u16; 4], s: u64, x: i32, y: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.2d.v4.b8.trap [%0, {%1, %2}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") y, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.2d.v4.b8.clamp [%0, {%1, %2}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") y, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.2d.v4.b8.zero [%0, {%1, %2}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") y, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
        }
    }
}

/// PTX `suld.b.2d.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_2d_b16_1(s: u64, x: i32, y: i32, m: Boundary) -> [u16; 1] {
    let a: u16;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.2d.b16.trap {%0}, [%1, {%2, %3}];", out("=h") a, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.2d.b16.clamp {%0}, [%1, {%2, %3}];", out("=h") a, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.2d.b16.zero {%0}, [%1, {%2, %3}];", out("=h") a, in("l") s, in("r") x, in("r") y)
            }
        }
    }
    [a]
}

/// PTX `sust.b.2d.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_2d_b16_1(v: [u16; 1], s: u64, x: i32, y: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.2d.b16.trap [%0, {%1, %2}], {%3};", in("l") s, in("r") x, in("r") y, in("h") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.2d.b16.clamp [%0, {%1, %2}], {%3};", in("l") s, in("r") x, in("r") y, in("h") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.2d.b16.zero [%0, {%1, %2}], {%3};", in("l") s, in("r") x, in("r") y, in("h") v[0])
            }
        }
    }
}

/// PTX `suld.b.2d.v2.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_2d_b16_2(s: u64, x: i32, y: i32, m: Boundary) -> [u16; 2] {
    let (a, b): (u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.2d.v2.b16.trap {%0, %1}, [%2, {%3, %4}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.2d.v2.b16.clamp {%0, %1}, [%2, {%3, %4}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.2d.v2.b16.zero {%0, %1}, [%2, {%3, %4}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.2d.v2.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_2d_b16_2(v: [u16; 2], s: u64, x: i32, y: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.2d.v2.b16.trap [%0, {%1, %2}], {%3, %4};", in("l") s, in("r") x, in("r") y, in("h") v[0], in("h") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.2d.v2.b16.clamp [%0, {%1, %2}], {%3, %4};", in("l") s, in("r") x, in("r") y, in("h") v[0], in("h") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.2d.v2.b16.zero [%0, {%1, %2}], {%3, %4};", in("l") s, in("r") x, in("r") y, in("h") v[0], in("h") v[1])
            }
        }
    }
}

/// PTX `suld.b.2d.v4.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_2d_b16_4(s: u64, x: i32, y: i32, m: Boundary) -> [u16; 4] {
    let (a, b, c, d): (u16, u16, u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.2d.v4.b16.trap {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.2d.v4.b16.clamp {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.2d.v4.b16.zero {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y)
            }
        }
    }
    [a, b, c, d]
}

/// PTX `sust.b.2d.v4.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_2d_b16_4(v: [u16; 4], s: u64, x: i32, y: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.2d.v4.b16.trap [%0, {%1, %2}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") y, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.2d.v4.b16.clamp [%0, {%1, %2}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") y, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.2d.v4.b16.zero [%0, {%1, %2}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") y, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
        }
    }
}

/// PTX `suld.b.2d.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_2d_b32_1(s: u64, x: i32, y: i32, m: Boundary) -> [u32; 1] {
    let a: u32;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.2d.b32.trap {%0}, [%1, {%2, %3}];", out("=r") a, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.2d.b32.clamp {%0}, [%1, {%2, %3}];", out("=r") a, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.2d.b32.zero {%0}, [%1, {%2, %3}];", out("=r") a, in("l") s, in("r") x, in("r") y)
            }
        }
    }
    [a]
}

/// PTX `sust.b.2d.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_2d_b32_1(v: [u32; 1], s: u64, x: i32, y: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.2d.b32.trap [%0, {%1, %2}], {%3};", in("l") s, in("r") x, in("r") y, in("r") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.2d.b32.clamp [%0, {%1, %2}], {%3};", in("l") s, in("r") x, in("r") y, in("r") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.2d.b32.zero [%0, {%1, %2}], {%3};", in("l") s, in("r") x, in("r") y, in("r") v[0])
            }
        }
    }
}

/// PTX `suld.b.2d.v2.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_2d_b32_2(s: u64, x: i32, y: i32, m: Boundary) -> [u32; 2] {
    let (a, b): (u32, u32);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.2d.v2.b32.trap {%0, %1}, [%2, {%3, %4}];", out("=r") a, out("=r") b, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.2d.v2.b32.clamp {%0, %1}, [%2, {%3, %4}];", out("=r") a, out("=r") b, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.2d.v2.b32.zero {%0, %1}, [%2, {%3, %4}];", out("=r") a, out("=r") b, in("l") s, in("r") x, in("r") y)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.2d.v2.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_2d_b32_2(v: [u32; 2], s: u64, x: i32, y: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.2d.v2.b32.trap [%0, {%1, %2}], {%3, %4};", in("l") s, in("r") x, in("r") y, in("r") v[0], in("r") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.2d.v2.b32.clamp [%0, {%1, %2}], {%3, %4};", in("l") s, in("r") x, in("r") y, in("r") v[0], in("r") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.2d.v2.b32.zero [%0, {%1, %2}], {%3, %4};", in("l") s, in("r") x, in("r") y, in("r") v[0], in("r") v[1])
            }
        }
    }
}

/// PTX `suld.b.2d.v4.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_2d_b32_4(s: u64, x: i32, y: i32, m: Boundary) -> [u32; 4] {
    let (a, b, c, d): (u32, u32, u32, u32);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.2d.v4.b32.trap {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.2d.v4.b32.clamp {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.2d.v4.b32.zero {%0, %1, %2, %3}, [%4, {%5, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") s, in("r") x, in("r") y)
            }
        }
    }
    [a, b, c, d]
}

/// PTX `sust.b.2d.v4.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_2d_b32_4(v: [u32; 4], s: u64, x: i32, y: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.2d.v4.b32.trap [%0, {%1, %2}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") y, in("r") v[0], in("r") v[1], in("r") v[2], in("r") v[3])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.2d.v4.b32.clamp [%0, {%1, %2}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") y, in("r") v[0], in("r") v[1], in("r") v[2], in("r") v[3])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.2d.v4.b32.zero [%0, {%1, %2}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") y, in("r") v[0], in("r") v[1], in("r") v[2], in("r") v[3])
            }
        }
    }
}

/// PTX `suld.b.2d.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_2d_b64_1(s: u64, x: i32, y: i32, m: Boundary) -> [u64; 1] {
    let a: u64;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.2d.b64.trap {%0}, [%1, {%2, %3}];", out("=l") a, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.2d.b64.clamp {%0}, [%1, {%2, %3}];", out("=l") a, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.2d.b64.zero {%0}, [%1, {%2, %3}];", out("=l") a, in("l") s, in("r") x, in("r") y)
            }
        }
    }
    [a]
}

/// PTX `sust.b.2d.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_2d_b64_1(v: [u64; 1], s: u64, x: i32, y: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.2d.b64.trap [%0, {%1, %2}], {%3};", in("l") s, in("r") x, in("r") y, in("l") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.2d.b64.clamp [%0, {%1, %2}], {%3};", in("l") s, in("r") x, in("r") y, in("l") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.2d.b64.zero [%0, {%1, %2}], {%3};", in("l") s, in("r") x, in("r") y, in("l") v[0])
            }
        }
    }
}

/// PTX `suld.b.2d.v2.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_2d_b64_2(s: u64, x: i32, y: i32, m: Boundary) -> [u64; 2] {
    let (a, b): (u64, u64);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.2d.v2.b64.trap {%0, %1}, [%2, {%3, %4}];", out("=l") a, out("=l") b, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.2d.v2.b64.clamp {%0, %1}, [%2, {%3, %4}];", out("=l") a, out("=l") b, in("l") s, in("r") x, in("r") y)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.2d.v2.b64.zero {%0, %1}, [%2, {%3, %4}];", out("=l") a, out("=l") b, in("l") s, in("r") x, in("r") y)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.2d.v2.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_2d_b64_2(v: [u64; 2], s: u64, x: i32, y: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.2d.v2.b64.trap [%0, {%1, %2}], {%3, %4};", in("l") s, in("r") x, in("r") y, in("l") v[0], in("l") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.2d.v2.b64.clamp [%0, {%1, %2}], {%3, %4};", in("l") s, in("r") x, in("r") y, in("l") v[0], in("l") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.2d.v2.b64.zero [%0, {%1, %2}], {%3, %4};", in("l") s, in("r") x, in("r") y, in("l") v[0], in("l") v[1])
            }
        }
    }
}

/// PTX `suld.b.3d.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_3d_b8_1(s: u64, x: i32, y: i32, z: i32, m: Boundary) -> [u16; 1] {
    let a: u16;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.3d.b8.trap {%0}, [%1, {%2, %3, %4, %4}];", out("=h") a, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.3d.b8.clamp {%0}, [%1, {%2, %3, %4, %4}];", out("=h") a, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.3d.b8.zero {%0}, [%1, {%2, %3, %4, %4}];", out("=h") a, in("l") s, in("r") x, in("r") y, in("r") z)
            }
        }
    }
    [a]
}

/// PTX `sust.b.3d.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_3d_b8_1(v: [u16; 1], s: u64, x: i32, y: i32, z: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.3d.b8.trap [%0, {%1, %2, %3, %3}], {%4};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.3d.b8.clamp [%0, {%1, %2, %3, %3}], {%4};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.3d.b8.zero [%0, {%1, %2, %3, %3}], {%4};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0])
            }
        }
    }
}

/// PTX `suld.b.3d.v2.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_3d_b8_2(s: u64, x: i32, y: i32, z: i32, m: Boundary) -> [u16; 2] {
    let (a, b): (u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.3d.v2.b8.trap {%0, %1}, [%2, {%3, %4, %5, %5}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.3d.v2.b8.clamp {%0, %1}, [%2, {%3, %4, %5, %5}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.3d.v2.b8.zero {%0, %1}, [%2, {%3, %4, %5, %5}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y, in("r") z)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.3d.v2.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_3d_b8_2(v: [u16; 2], s: u64, x: i32, y: i32, z: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.3d.v2.b8.trap [%0, {%1, %2, %3, %3}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0], in("h") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.3d.v2.b8.clamp [%0, {%1, %2, %3, %3}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0], in("h") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.3d.v2.b8.zero [%0, {%1, %2, %3, %3}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0], in("h") v[1])
            }
        }
    }
}

/// PTX `suld.b.3d.v4.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_3d_b8_4(s: u64, x: i32, y: i32, z: i32, m: Boundary) -> [u16; 4] {
    let (a, b, c, d): (u16, u16, u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.3d.v4.b8.trap {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.3d.v4.b8.clamp {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.3d.v4.b8.zero {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y, in("r") z)
            }
        }
    }
    [a, b, c, d]
}

/// PTX `sust.b.3d.v4.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_3d_b8_4(v: [u16; 4], s: u64, x: i32, y: i32, z: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.3d.v4.b8.trap [%0, {%1, %2, %3, %3}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.3d.v4.b8.clamp [%0, {%1, %2, %3, %3}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.3d.v4.b8.zero [%0, {%1, %2, %3, %3}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
        }
    }
}

/// PTX `suld.b.3d.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_3d_b16_1(s: u64, x: i32, y: i32, z: i32, m: Boundary) -> [u16; 1] {
    let a: u16;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.3d.b16.trap {%0}, [%1, {%2, %3, %4, %4}];", out("=h") a, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.3d.b16.clamp {%0}, [%1, {%2, %3, %4, %4}];", out("=h") a, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.3d.b16.zero {%0}, [%1, {%2, %3, %4, %4}];", out("=h") a, in("l") s, in("r") x, in("r") y, in("r") z)
            }
        }
    }
    [a]
}

/// PTX `sust.b.3d.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_3d_b16_1(v: [u16; 1], s: u64, x: i32, y: i32, z: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.3d.b16.trap [%0, {%1, %2, %3, %3}], {%4};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.3d.b16.clamp [%0, {%1, %2, %3, %3}], {%4};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.3d.b16.zero [%0, {%1, %2, %3, %3}], {%4};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0])
            }
        }
    }
}

/// PTX `suld.b.3d.v2.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_3d_b16_2(s: u64, x: i32, y: i32, z: i32, m: Boundary) -> [u16; 2] {
    let (a, b): (u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.3d.v2.b16.trap {%0, %1}, [%2, {%3, %4, %5, %5}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.3d.v2.b16.clamp {%0, %1}, [%2, {%3, %4, %5, %5}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.3d.v2.b16.zero {%0, %1}, [%2, {%3, %4, %5, %5}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y, in("r") z)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.3d.v2.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_3d_b16_2(v: [u16; 2], s: u64, x: i32, y: i32, z: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.3d.v2.b16.trap [%0, {%1, %2, %3, %3}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0], in("h") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.3d.v2.b16.clamp [%0, {%1, %2, %3, %3}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0], in("h") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.3d.v2.b16.zero [%0, {%1, %2, %3, %3}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0], in("h") v[1])
            }
        }
    }
}

/// PTX `suld.b.3d.v4.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_3d_b16_4(s: u64, x: i32, y: i32, z: i32, m: Boundary) -> [u16; 4] {
    let (a, b, c, d): (u16, u16, u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.3d.v4.b16.trap {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.3d.v4.b16.clamp {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.3d.v4.b16.zero {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y, in("r") z)
            }
        }
    }
    [a, b, c, d]
}

/// PTX `sust.b.3d.v4.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_3d_b16_4(v: [u16; 4], s: u64, x: i32, y: i32, z: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.3d.v4.b16.trap [%0, {%1, %2, %3, %3}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.3d.v4.b16.clamp [%0, {%1, %2, %3, %3}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.3d.v4.b16.zero [%0, {%1, %2, %3, %3}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") z, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
        }
    }
}

/// PTX `suld.b.3d.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_3d_b32_1(s: u64, x: i32, y: i32, z: i32, m: Boundary) -> [u32; 1] {
    let a: u32;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.3d.b32.trap {%0}, [%1, {%2, %3, %4, %4}];", out("=r") a, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.3d.b32.clamp {%0}, [%1, {%2, %3, %4, %4}];", out("=r") a, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.3d.b32.zero {%0}, [%1, {%2, %3, %4, %4}];", out("=r") a, in("l") s, in("r") x, in("r") y, in("r") z)
            }
        }
    }
    [a]
}

/// PTX `sust.b.3d.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_3d_b32_1(v: [u32; 1], s: u64, x: i32, y: i32, z: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.3d.b32.trap [%0, {%1, %2, %3, %3}], {%4};", in("l") s, in("r") x, in("r") y, in("r") z, in("r") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.3d.b32.clamp [%0, {%1, %2, %3, %3}], {%4};", in("l") s, in("r") x, in("r") y, in("r") z, in("r") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.3d.b32.zero [%0, {%1, %2, %3, %3}], {%4};", in("l") s, in("r") x, in("r") y, in("r") z, in("r") v[0])
            }
        }
    }
}

/// PTX `suld.b.3d.v2.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_3d_b32_2(s: u64, x: i32, y: i32, z: i32, m: Boundary) -> [u32; 2] {
    let (a, b): (u32, u32);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.3d.v2.b32.trap {%0, %1}, [%2, {%3, %4, %5, %5}];", out("=r") a, out("=r") b, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.3d.v2.b32.clamp {%0, %1}, [%2, {%3, %4, %5, %5}];", out("=r") a, out("=r") b, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.3d.v2.b32.zero {%0, %1}, [%2, {%3, %4, %5, %5}];", out("=r") a, out("=r") b, in("l") s, in("r") x, in("r") y, in("r") z)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.3d.v2.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_3d_b32_2(v: [u32; 2], s: u64, x: i32, y: i32, z: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.3d.v2.b32.trap [%0, {%1, %2, %3, %3}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") z, in("r") v[0], in("r") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.3d.v2.b32.clamp [%0, {%1, %2, %3, %3}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") z, in("r") v[0], in("r") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.3d.v2.b32.zero [%0, {%1, %2, %3, %3}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") z, in("r") v[0], in("r") v[1])
            }
        }
    }
}

/// PTX `suld.b.3d.v4.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_3d_b32_4(s: u64, x: i32, y: i32, z: i32, m: Boundary) -> [u32; 4] {
    let (a, b, c, d): (u32, u32, u32, u32);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.3d.v4.b32.trap {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.3d.v4.b32.clamp {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.3d.v4.b32.zero {%0, %1, %2, %3}, [%4, {%5, %6, %7, %7}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") s, in("r") x, in("r") y, in("r") z)
            }
        }
    }
    [a, b, c, d]
}

/// PTX `sust.b.3d.v4.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_3d_b32_4(v: [u32; 4], s: u64, x: i32, y: i32, z: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.3d.v4.b32.trap [%0, {%1, %2, %3, %3}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") z, in("r") v[0], in("r") v[1], in("r") v[2], in("r") v[3])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.3d.v4.b32.clamp [%0, {%1, %2, %3, %3}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") z, in("r") v[0], in("r") v[1], in("r") v[2], in("r") v[3])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.3d.v4.b32.zero [%0, {%1, %2, %3, %3}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") z, in("r") v[0], in("r") v[1], in("r") v[2], in("r") v[3])
            }
        }
    }
}

/// PTX `suld.b.3d.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_3d_b64_1(s: u64, x: i32, y: i32, z: i32, m: Boundary) -> [u64; 1] {
    let a: u64;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.3d.b64.trap {%0}, [%1, {%2, %3, %4, %4}];", out("=l") a, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.3d.b64.clamp {%0}, [%1, {%2, %3, %4, %4}];", out("=l") a, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.3d.b64.zero {%0}, [%1, {%2, %3, %4, %4}];", out("=l") a, in("l") s, in("r") x, in("r") y, in("r") z)
            }
        }
    }
    [a]
}

/// PTX `sust.b.3d.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_3d_b64_1(v: [u64; 1], s: u64, x: i32, y: i32, z: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.3d.b64.trap [%0, {%1, %2, %3, %3}], {%4};", in("l") s, in("r") x, in("r") y, in("r") z, in("l") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.3d.b64.clamp [%0, {%1, %2, %3, %3}], {%4};", in("l") s, in("r") x, in("r") y, in("r") z, in("l") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.3d.b64.zero [%0, {%1, %2, %3, %3}], {%4};", in("l") s, in("r") x, in("r") y, in("r") z, in("l") v[0])
            }
        }
    }
}

/// PTX `suld.b.3d.v2.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_3d_b64_2(s: u64, x: i32, y: i32, z: i32, m: Boundary) -> [u64; 2] {
    let (a, b): (u64, u64);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.3d.v2.b64.trap {%0, %1}, [%2, {%3, %4, %5, %5}];", out("=l") a, out("=l") b, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.3d.v2.b64.clamp {%0, %1}, [%2, {%3, %4, %5, %5}];", out("=l") a, out("=l") b, in("l") s, in("r") x, in("r") y, in("r") z)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.3d.v2.b64.zero {%0, %1}, [%2, {%3, %4, %5, %5}];", out("=l") a, out("=l") b, in("l") s, in("r") x, in("r") y, in("r") z)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.3d.v2.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_3d_b64_2(v: [u64; 2], s: u64, x: i32, y: i32, z: i32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.3d.v2.b64.trap [%0, {%1, %2, %3, %3}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") z, in("l") v[0], in("l") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.3d.v2.b64.clamp [%0, {%1, %2, %3, %3}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") z, in("l") v[0], in("l") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.3d.v2.b64.zero [%0, {%1, %2, %3, %3}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") z, in("l") v[0], in("l") v[1])
            }
        }
    }
}

/// PTX `suld.b.a1d.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a1d_b8_1(s: u64, x: i32, layer: u32, m: Boundary) -> [u16; 1] {
    let a: u16;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a1d.b8.trap {%0}, [%1, {%3, %2}];", out("=h") a, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a1d.b8.clamp {%0}, [%1, {%3, %2}];", out("=h") a, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a1d.b8.zero {%0}, [%1, {%3, %2}];", out("=h") a, in("l") s, in("r") x, in("r") layer)
            }
        }
    }
    [a]
}

/// PTX `sust.b.a1d.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a1d_b8_1(v: [u16; 1], s: u64, x: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a1d.b8.trap [%0, {%2, %1}], {%3};", in("l") s, in("r") x, in("r") layer, in("h") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a1d.b8.clamp [%0, {%2, %1}], {%3};", in("l") s, in("r") x, in("r") layer, in("h") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a1d.b8.zero [%0, {%2, %1}], {%3};", in("l") s, in("r") x, in("r") layer, in("h") v[0])
            }
        }
    }
}

/// PTX `suld.b.a1d.v2.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a1d_b8_2(s: u64, x: i32, layer: u32, m: Boundary) -> [u16; 2] {
    let (a, b): (u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a1d.v2.b8.trap {%0, %1}, [%2, {%4, %3}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a1d.v2.b8.clamp {%0, %1}, [%2, {%4, %3}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a1d.v2.b8.zero {%0, %1}, [%2, {%4, %3}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") layer)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.a1d.v2.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a1d_b8_2(v: [u16; 2], s: u64, x: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a1d.v2.b8.trap [%0, {%2, %1}], {%3, %4};", in("l") s, in("r") x, in("r") layer, in("h") v[0], in("h") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a1d.v2.b8.clamp [%0, {%2, %1}], {%3, %4};", in("l") s, in("r") x, in("r") layer, in("h") v[0], in("h") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a1d.v2.b8.zero [%0, {%2, %1}], {%3, %4};", in("l") s, in("r") x, in("r") layer, in("h") v[0], in("h") v[1])
            }
        }
    }
}

/// PTX `suld.b.a1d.v4.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a1d_b8_4(s: u64, x: i32, layer: u32, m: Boundary) -> [u16; 4] {
    let (a, b, c, d): (u16, u16, u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a1d.v4.b8.trap {%0, %1, %2, %3}, [%4, {%6, %5}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a1d.v4.b8.clamp {%0, %1, %2, %3}, [%4, {%6, %5}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a1d.v4.b8.zero {%0, %1, %2, %3}, [%4, {%6, %5}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") layer)
            }
        }
    }
    [a, b, c, d]
}

/// PTX `sust.b.a1d.v4.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a1d_b8_4(v: [u16; 4], s: u64, x: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a1d.v4.b8.trap [%0, {%2, %1}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") layer, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a1d.v4.b8.clamp [%0, {%2, %1}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") layer, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a1d.v4.b8.zero [%0, {%2, %1}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") layer, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
        }
    }
}

/// PTX `suld.b.a1d.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a1d_b16_1(s: u64, x: i32, layer: u32, m: Boundary) -> [u16; 1] {
    let a: u16;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a1d.b16.trap {%0}, [%1, {%3, %2}];", out("=h") a, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a1d.b16.clamp {%0}, [%1, {%3, %2}];", out("=h") a, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a1d.b16.zero {%0}, [%1, {%3, %2}];", out("=h") a, in("l") s, in("r") x, in("r") layer)
            }
        }
    }
    [a]
}

/// PTX `sust.b.a1d.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a1d_b16_1(v: [u16; 1], s: u64, x: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a1d.b16.trap [%0, {%2, %1}], {%3};", in("l") s, in("r") x, in("r") layer, in("h") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a1d.b16.clamp [%0, {%2, %1}], {%3};", in("l") s, in("r") x, in("r") layer, in("h") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a1d.b16.zero [%0, {%2, %1}], {%3};", in("l") s, in("r") x, in("r") layer, in("h") v[0])
            }
        }
    }
}

/// PTX `suld.b.a1d.v2.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a1d_b16_2(s: u64, x: i32, layer: u32, m: Boundary) -> [u16; 2] {
    let (a, b): (u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a1d.v2.b16.trap {%0, %1}, [%2, {%4, %3}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a1d.v2.b16.clamp {%0, %1}, [%2, {%4, %3}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a1d.v2.b16.zero {%0, %1}, [%2, {%4, %3}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") layer)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.a1d.v2.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a1d_b16_2(v: [u16; 2], s: u64, x: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a1d.v2.b16.trap [%0, {%2, %1}], {%3, %4};", in("l") s, in("r") x, in("r") layer, in("h") v[0], in("h") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a1d.v2.b16.clamp [%0, {%2, %1}], {%3, %4};", in("l") s, in("r") x, in("r") layer, in("h") v[0], in("h") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a1d.v2.b16.zero [%0, {%2, %1}], {%3, %4};", in("l") s, in("r") x, in("r") layer, in("h") v[0], in("h") v[1])
            }
        }
    }
}

/// PTX `suld.b.a1d.v4.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a1d_b16_4(s: u64, x: i32, layer: u32, m: Boundary) -> [u16; 4] {
    let (a, b, c, d): (u16, u16, u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a1d.v4.b16.trap {%0, %1, %2, %3}, [%4, {%6, %5}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a1d.v4.b16.clamp {%0, %1, %2, %3}, [%4, {%6, %5}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a1d.v4.b16.zero {%0, %1, %2, %3}, [%4, {%6, %5}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") layer)
            }
        }
    }
    [a, b, c, d]
}

/// PTX `sust.b.a1d.v4.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a1d_b16_4(v: [u16; 4], s: u64, x: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a1d.v4.b16.trap [%0, {%2, %1}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") layer, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a1d.v4.b16.clamp [%0, {%2, %1}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") layer, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a1d.v4.b16.zero [%0, {%2, %1}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") layer, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
        }
    }
}

/// PTX `suld.b.a1d.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a1d_b32_1(s: u64, x: i32, layer: u32, m: Boundary) -> [u32; 1] {
    let a: u32;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a1d.b32.trap {%0}, [%1, {%3, %2}];", out("=r") a, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a1d.b32.clamp {%0}, [%1, {%3, %2}];", out("=r") a, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a1d.b32.zero {%0}, [%1, {%3, %2}];", out("=r") a, in("l") s, in("r") x, in("r") layer)
            }
        }
    }
    [a]
}

/// PTX `sust.b.a1d.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a1d_b32_1(v: [u32; 1], s: u64, x: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a1d.b32.trap [%0, {%2, %1}], {%3};", in("l") s, in("r") x, in("r") layer, in("r") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a1d.b32.clamp [%0, {%2, %1}], {%3};", in("l") s, in("r") x, in("r") layer, in("r") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a1d.b32.zero [%0, {%2, %1}], {%3};", in("l") s, in("r") x, in("r") layer, in("r") v[0])
            }
        }
    }
}

/// PTX `suld.b.a1d.v2.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a1d_b32_2(s: u64, x: i32, layer: u32, m: Boundary) -> [u32; 2] {
    let (a, b): (u32, u32);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a1d.v2.b32.trap {%0, %1}, [%2, {%4, %3}];", out("=r") a, out("=r") b, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a1d.v2.b32.clamp {%0, %1}, [%2, {%4, %3}];", out("=r") a, out("=r") b, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a1d.v2.b32.zero {%0, %1}, [%2, {%4, %3}];", out("=r") a, out("=r") b, in("l") s, in("r") x, in("r") layer)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.a1d.v2.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a1d_b32_2(v: [u32; 2], s: u64, x: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a1d.v2.b32.trap [%0, {%2, %1}], {%3, %4};", in("l") s, in("r") x, in("r") layer, in("r") v[0], in("r") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a1d.v2.b32.clamp [%0, {%2, %1}], {%3, %4};", in("l") s, in("r") x, in("r") layer, in("r") v[0], in("r") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a1d.v2.b32.zero [%0, {%2, %1}], {%3, %4};", in("l") s, in("r") x, in("r") layer, in("r") v[0], in("r") v[1])
            }
        }
    }
}

/// PTX `suld.b.a1d.v4.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a1d_b32_4(s: u64, x: i32, layer: u32, m: Boundary) -> [u32; 4] {
    let (a, b, c, d): (u32, u32, u32, u32);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a1d.v4.b32.trap {%0, %1, %2, %3}, [%4, {%6, %5}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a1d.v4.b32.clamp {%0, %1, %2, %3}, [%4, {%6, %5}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a1d.v4.b32.zero {%0, %1, %2, %3}, [%4, {%6, %5}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") s, in("r") x, in("r") layer)
            }
        }
    }
    [a, b, c, d]
}

/// PTX `sust.b.a1d.v4.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a1d_b32_4(v: [u32; 4], s: u64, x: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a1d.v4.b32.trap [%0, {%2, %1}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") layer, in("r") v[0], in("r") v[1], in("r") v[2], in("r") v[3])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a1d.v4.b32.clamp [%0, {%2, %1}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") layer, in("r") v[0], in("r") v[1], in("r") v[2], in("r") v[3])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a1d.v4.b32.zero [%0, {%2, %1}], {%3, %4, %5, %6};", in("l") s, in("r") x, in("r") layer, in("r") v[0], in("r") v[1], in("r") v[2], in("r") v[3])
            }
        }
    }
}

/// PTX `suld.b.a1d.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a1d_b64_1(s: u64, x: i32, layer: u32, m: Boundary) -> [u64; 1] {
    let a: u64;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a1d.b64.trap {%0}, [%1, {%3, %2}];", out("=l") a, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a1d.b64.clamp {%0}, [%1, {%3, %2}];", out("=l") a, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a1d.b64.zero {%0}, [%1, {%3, %2}];", out("=l") a, in("l") s, in("r") x, in("r") layer)
            }
        }
    }
    [a]
}

/// PTX `sust.b.a1d.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a1d_b64_1(v: [u64; 1], s: u64, x: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a1d.b64.trap [%0, {%2, %1}], {%3};", in("l") s, in("r") x, in("r") layer, in("l") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a1d.b64.clamp [%0, {%2, %1}], {%3};", in("l") s, in("r") x, in("r") layer, in("l") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a1d.b64.zero [%0, {%2, %1}], {%3};", in("l") s, in("r") x, in("r") layer, in("l") v[0])
            }
        }
    }
}

/// PTX `suld.b.a1d.v2.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a1d_b64_2(s: u64, x: i32, layer: u32, m: Boundary) -> [u64; 2] {
    let (a, b): (u64, u64);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a1d.v2.b64.trap {%0, %1}, [%2, {%4, %3}];", out("=l") a, out("=l") b, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a1d.v2.b64.clamp {%0, %1}, [%2, {%4, %3}];", out("=l") a, out("=l") b, in("l") s, in("r") x, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a1d.v2.b64.zero {%0, %1}, [%2, {%4, %3}];", out("=l") a, out("=l") b, in("l") s, in("r") x, in("r") layer)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.a1d.v2.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a1d_b64_2(v: [u64; 2], s: u64, x: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a1d.v2.b64.trap [%0, {%2, %1}], {%3, %4};", in("l") s, in("r") x, in("r") layer, in("l") v[0], in("l") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a1d.v2.b64.clamp [%0, {%2, %1}], {%3, %4};", in("l") s, in("r") x, in("r") layer, in("l") v[0], in("l") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a1d.v2.b64.zero [%0, {%2, %1}], {%3, %4};", in("l") s, in("r") x, in("r") layer, in("l") v[0], in("l") v[1])
            }
        }
    }
}

/// PTX `suld.b.a2d.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a2d_b8_1(s: u64, x: i32, y: i32, layer: u32, m: Boundary) -> [u16; 1] {
    let a: u16;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a2d.b8.trap {%0}, [%1, {%4, %2, %3, %3}];", out("=h") a, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a2d.b8.clamp {%0}, [%1, {%4, %2, %3, %3}];", out("=h") a, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a2d.b8.zero {%0}, [%1, {%4, %2, %3, %3}];", out("=h") a, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
        }
    }
    [a]
}

/// PTX `sust.b.a2d.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a2d_b8_1(v: [u16; 1], s: u64, x: i32, y: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a2d.b8.trap [%0, {%3, %1, %2, %2}], {%4};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a2d.b8.clamp [%0, {%3, %1, %2, %2}], {%4};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a2d.b8.zero [%0, {%3, %1, %2, %2}], {%4};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0])
            }
        }
    }
}

/// PTX `suld.b.a2d.v2.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a2d_b8_2(s: u64, x: i32, y: i32, layer: u32, m: Boundary) -> [u16; 2] {
    let (a, b): (u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a2d.v2.b8.trap {%0, %1}, [%2, {%5, %3, %4, %4}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a2d.v2.b8.clamp {%0, %1}, [%2, {%5, %3, %4, %4}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a2d.v2.b8.zero {%0, %1}, [%2, {%5, %3, %4, %4}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.a2d.v2.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a2d_b8_2(v: [u16; 2], s: u64, x: i32, y: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a2d.v2.b8.trap [%0, {%3, %1, %2, %2}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0], in("h") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a2d.v2.b8.clamp [%0, {%3, %1, %2, %2}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0], in("h") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a2d.v2.b8.zero [%0, {%3, %1, %2, %2}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0], in("h") v[1])
            }
        }
    }
}

/// PTX `suld.b.a2d.v4.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a2d_b8_4(s: u64, x: i32, y: i32, layer: u32, m: Boundary) -> [u16; 4] {
    let (a, b, c, d): (u16, u16, u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a2d.v4.b8.trap {%0, %1, %2, %3}, [%4, {%7, %5, %6, %6}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a2d.v4.b8.clamp {%0, %1, %2, %3}, [%4, {%7, %5, %6, %6}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a2d.v4.b8.zero {%0, %1, %2, %3}, [%4, {%7, %5, %6, %6}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
        }
    }
    [a, b, c, d]
}

/// PTX `sust.b.a2d.v4.b8.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a2d_b8_4(v: [u16; 4], s: u64, x: i32, y: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a2d.v4.b8.trap [%0, {%3, %1, %2, %2}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a2d.v4.b8.clamp [%0, {%3, %1, %2, %2}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a2d.v4.b8.zero [%0, {%3, %1, %2, %2}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
        }
    }
}

/// PTX `suld.b.a2d.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a2d_b16_1(s: u64, x: i32, y: i32, layer: u32, m: Boundary) -> [u16; 1] {
    let a: u16;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a2d.b16.trap {%0}, [%1, {%4, %2, %3, %3}];", out("=h") a, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a2d.b16.clamp {%0}, [%1, {%4, %2, %3, %3}];", out("=h") a, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a2d.b16.zero {%0}, [%1, {%4, %2, %3, %3}];", out("=h") a, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
        }
    }
    [a]
}

/// PTX `sust.b.a2d.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a2d_b16_1(v: [u16; 1], s: u64, x: i32, y: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a2d.b16.trap [%0, {%3, %1, %2, %2}], {%4};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a2d.b16.clamp [%0, {%3, %1, %2, %2}], {%4};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a2d.b16.zero [%0, {%3, %1, %2, %2}], {%4};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0])
            }
        }
    }
}

/// PTX `suld.b.a2d.v2.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a2d_b16_2(s: u64, x: i32, y: i32, layer: u32, m: Boundary) -> [u16; 2] {
    let (a, b): (u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a2d.v2.b16.trap {%0, %1}, [%2, {%5, %3, %4, %4}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a2d.v2.b16.clamp {%0, %1}, [%2, {%5, %3, %4, %4}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a2d.v2.b16.zero {%0, %1}, [%2, {%5, %3, %4, %4}];", out("=h") a, out("=h") b, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.a2d.v2.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a2d_b16_2(v: [u16; 2], s: u64, x: i32, y: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a2d.v2.b16.trap [%0, {%3, %1, %2, %2}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0], in("h") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a2d.v2.b16.clamp [%0, {%3, %1, %2, %2}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0], in("h") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a2d.v2.b16.zero [%0, {%3, %1, %2, %2}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0], in("h") v[1])
            }
        }
    }
}

/// PTX `suld.b.a2d.v4.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a2d_b16_4(s: u64, x: i32, y: i32, layer: u32, m: Boundary) -> [u16; 4] {
    let (a, b, c, d): (u16, u16, u16, u16);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a2d.v4.b16.trap {%0, %1, %2, %3}, [%4, {%7, %5, %6, %6}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a2d.v4.b16.clamp {%0, %1, %2, %3}, [%4, {%7, %5, %6, %6}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a2d.v4.b16.zero {%0, %1, %2, %3}, [%4, {%7, %5, %6, %6}];", out("=h") a, out("=h") b, out("=h") c, out("=h") d, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
        }
    }
    [a, b, c, d]
}

/// PTX `sust.b.a2d.v4.b16.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a2d_b16_4(v: [u16; 4], s: u64, x: i32, y: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a2d.v4.b16.trap [%0, {%3, %1, %2, %2}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a2d.v4.b16.clamp [%0, {%3, %1, %2, %2}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a2d.v4.b16.zero [%0, {%3, %1, %2, %2}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") layer, in("h") v[0], in("h") v[1], in("h") v[2], in("h") v[3])
            }
        }
    }
}

/// PTX `suld.b.a2d.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a2d_b32_1(s: u64, x: i32, y: i32, layer: u32, m: Boundary) -> [u32; 1] {
    let a: u32;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a2d.b32.trap {%0}, [%1, {%4, %2, %3, %3}];", out("=r") a, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a2d.b32.clamp {%0}, [%1, {%4, %2, %3, %3}];", out("=r") a, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a2d.b32.zero {%0}, [%1, {%4, %2, %3, %3}];", out("=r") a, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
        }
    }
    [a]
}

/// PTX `sust.b.a2d.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a2d_b32_1(v: [u32; 1], s: u64, x: i32, y: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a2d.b32.trap [%0, {%3, %1, %2, %2}], {%4};", in("l") s, in("r") x, in("r") y, in("r") layer, in("r") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a2d.b32.clamp [%0, {%3, %1, %2, %2}], {%4};", in("l") s, in("r") x, in("r") y, in("r") layer, in("r") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a2d.b32.zero [%0, {%3, %1, %2, %2}], {%4};", in("l") s, in("r") x, in("r") y, in("r") layer, in("r") v[0])
            }
        }
    }
}

/// PTX `suld.b.a2d.v2.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a2d_b32_2(s: u64, x: i32, y: i32, layer: u32, m: Boundary) -> [u32; 2] {
    let (a, b): (u32, u32);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a2d.v2.b32.trap {%0, %1}, [%2, {%5, %3, %4, %4}];", out("=r") a, out("=r") b, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a2d.v2.b32.clamp {%0, %1}, [%2, {%5, %3, %4, %4}];", out("=r") a, out("=r") b, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a2d.v2.b32.zero {%0, %1}, [%2, {%5, %3, %4, %4}];", out("=r") a, out("=r") b, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.a2d.v2.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a2d_b32_2(v: [u32; 2], s: u64, x: i32, y: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a2d.v2.b32.trap [%0, {%3, %1, %2, %2}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") layer, in("r") v[0], in("r") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a2d.v2.b32.clamp [%0, {%3, %1, %2, %2}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") layer, in("r") v[0], in("r") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a2d.v2.b32.zero [%0, {%3, %1, %2, %2}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") layer, in("r") v[0], in("r") v[1])
            }
        }
    }
}

/// PTX `suld.b.a2d.v4.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a2d_b32_4(s: u64, x: i32, y: i32, layer: u32, m: Boundary) -> [u32; 4] {
    let (a, b, c, d): (u32, u32, u32, u32);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a2d.v4.b32.trap {%0, %1, %2, %3}, [%4, {%7, %5, %6, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a2d.v4.b32.clamp {%0, %1, %2, %3}, [%4, {%7, %5, %6, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a2d.v4.b32.zero {%0, %1, %2, %3}, [%4, {%7, %5, %6, %6}];", out("=r") a, out("=r") b, out("=r") c, out("=r") d, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
        }
    }
    [a, b, c, d]
}

/// PTX `sust.b.a2d.v4.b32.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a2d_b32_4(v: [u32; 4], s: u64, x: i32, y: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a2d.v4.b32.trap [%0, {%3, %1, %2, %2}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") layer, in("r") v[0], in("r") v[1], in("r") v[2], in("r") v[3])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a2d.v4.b32.clamp [%0, {%3, %1, %2, %2}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") layer, in("r") v[0], in("r") v[1], in("r") v[2], in("r") v[3])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a2d.v4.b32.zero [%0, {%3, %1, %2, %2}], {%4, %5, %6, %7};", in("l") s, in("r") x, in("r") y, in("r") layer, in("r") v[0], in("r") v[1], in("r") v[2], in("r") v[3])
            }
        }
    }
}

/// PTX `suld.b.a2d.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a2d_b64_1(s: u64, x: i32, y: i32, layer: u32, m: Boundary) -> [u64; 1] {
    let a: u64;
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a2d.b64.trap {%0}, [%1, {%4, %2, %3, %3}];", out("=l") a, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a2d.b64.clamp {%0}, [%1, {%4, %2, %3, %3}];", out("=l") a, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a2d.b64.zero {%0}, [%1, {%4, %2, %3, %3}];", out("=l") a, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
        }
    }
    [a]
}

/// PTX `sust.b.a2d.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a2d_b64_1(v: [u64; 1], s: u64, x: i32, y: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a2d.b64.trap [%0, {%3, %1, %2, %2}], {%4};", in("l") s, in("r") x, in("r") y, in("r") layer, in("l") v[0])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a2d.b64.clamp [%0, {%3, %1, %2, %2}], {%4};", in("l") s, in("r") x, in("r") y, in("r") layer, in("l") v[0])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a2d.b64.zero [%0, {%3, %1, %2, %2}], {%4};", in("l") s, in("r") x, in("r") y, in("r") layer, in("l") v[0])
            }
        }
    }
}

/// PTX `suld.b.a2d.v2.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn suld_a2d_b64_2(s: u64, x: i32, y: i32, layer: u32, m: Boundary) -> [u64; 2] {
    let (a, b): (u64, u64);
    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered
    // after this thread's earlier `sust` stores to the same surface.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("suld.b.a2d.v2.b64.trap {%0, %1}, [%2, {%5, %3, %4, %4}];", out("=l") a, out("=l") b, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Clamp => {
                ptx_asm!("suld.b.a2d.v2.b64.clamp {%0, %1}, [%2, {%5, %3, %4, %4}];", out("=l") a, out("=l") b, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
            Boundary::Zero => {
                ptx_asm!("suld.b.a2d.v2.b64.zero {%0, %1}, [%2, {%5, %3, %4, %4}];", out("=l") a, out("=l") b, in("l") s, in("r") x, in("r") y, in("r") layer)
            }
        }
    }
    [a, b]
}

/// PTX `sust.b.a2d.v2.b64.{trap,clamp,zero}`.
///
/// # Safety
///
/// A live surface object of this shape whose element has this layout.
#[inline(always)]
unsafe fn sust_a2d_b64_2(v: [u64; 2], s: u64, x: i32, y: i32, layer: u32, m: Boundary) {
    // SAFETY: forwarded contract.
    unsafe {
        match m {
            Boundary::Trap => {
                ptx_asm!("sust.b.a2d.v2.b64.trap [%0, {%3, %1, %2, %2}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") layer, in("l") v[0], in("l") v[1])
            }
            Boundary::Clamp => {
                ptx_asm!("sust.b.a2d.v2.b64.clamp [%0, {%3, %1, %2, %2}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") layer, in("l") v[0], in("l") v[1])
            }
            Boundary::Zero => {
                ptx_asm!("sust.b.a2d.v2.b64.zero [%0, {%3, %1, %2, %2}], {%4, %5};", in("l") s, in("r") x, in("r") y, in("r") layer, in("l") v[0], in("l") v[1])
            }
        }
    }
}

/// `surf1Dread<T>(surf, ..., mode)`: PTX `suld.b.1d.{b8..v4.b32,b64,v2.b64}.{trap,clamp,zero}`.
///
/// # Safety
///
/// `surf` must be a live surface object over a 1D array.
/// Its element must have `T`'s size and `x` is in bytes (see [`SurfElem`]). With
/// [`Boundary::Trap`] an out-of-range coordinate aborts the launch.
#[inline(always)]
pub unsafe fn surf1d_read_mode<T: SurfElem>(surf: SurfObject, x: i32, mode: Boundary) -> T {
    // SAFETY: forwarded contract; `T::BITS` and `T::N` are constants, so one arm survives.
    let w = unsafe {
        match (T::BITS, T::N) {
            (8, 1) => {
                let [a] = suld_1d_b8_1(surf, x, mode);
                [u64::from(a), 0, 0, 0]
            }
            (8, 2) => {
                let [a, b] = suld_1d_b8_2(surf, x, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (8, 4) => {
                let [a, b, c, d] = suld_1d_b8_4(surf, x, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (16, 1) => {
                let [a] = suld_1d_b16_1(surf, x, mode);
                [u64::from(a), 0, 0, 0]
            }
            (16, 2) => {
                let [a, b] = suld_1d_b16_2(surf, x, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (16, 4) => {
                let [a, b, c, d] = suld_1d_b16_4(surf, x, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (32, 1) => {
                let [a] = suld_1d_b32_1(surf, x, mode);
                [u64::from(a), 0, 0, 0]
            }
            (32, 2) => {
                let [a, b] = suld_1d_b32_2(surf, x, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (32, 4) => {
                let [a, b, c, d] = suld_1d_b32_4(surf, x, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (64, 1) => {
                let [a] = suld_1d_b64_1(surf, x, mode);
                [a, 0, 0, 0]
            }
            _ => {
                let [a, b] = suld_1d_b64_2(surf, x, mode);
                [a, b, 0, 0]
            }
        }
    };
    T::from_bits(w)
}

/// `surf1Dread<T>(surf, ...)`: [`surf1d_read_mode`] with CUDA's default [`Boundary::Trap`].
///
/// # Safety
///
/// As [`surf1d_read_mode`].
#[inline(always)]
pub unsafe fn surf1d_read<T: SurfElem>(surf: SurfObject, x: i32) -> T {
    // SAFETY: forwarded contract.
    unsafe { surf1d_read_mode(surf, x, Boundary::Trap) }
}

/// `surf1Dwrite<T>(v, surf, ..., mode)`: PTX `sust.b.1d.{b8..v4.b32,b64,v2.b64}.{trap,clamp,zero}`.
///
/// # Safety
///
/// As [`surf1d_read_mode`].
#[inline(always)]
pub unsafe fn surf1d_write_mode<T: SurfElem>(v: T, surf: SurfObject, x: i32, mode: Boundary) {
    let w = v.to_bits();
    // SAFETY: forwarded contract; `T::BITS` and `T::N` are constants, so one arm survives; the
    // narrowing casts keep exactly the component bits `to_bits` zero-extended.
    unsafe {
        match (T::BITS, T::N) {
            (8, 1) => sust_1d_b8_1([w[0] as u16], surf, x, mode),
            (8, 2) => sust_1d_b8_2([w[0] as u16, w[1] as u16], surf, x, mode),
            (8, 4) => sust_1d_b8_4(
                [w[0] as u16, w[1] as u16, w[2] as u16, w[3] as u16],
                surf,
                x,
                mode,
            ),
            (16, 1) => sust_1d_b16_1([w[0] as u16], surf, x, mode),
            (16, 2) => sust_1d_b16_2([w[0] as u16, w[1] as u16], surf, x, mode),
            (16, 4) => sust_1d_b16_4(
                [w[0] as u16, w[1] as u16, w[2] as u16, w[3] as u16],
                surf,
                x,
                mode,
            ),
            (32, 1) => sust_1d_b32_1([w[0] as u32], surf, x, mode),
            (32, 2) => sust_1d_b32_2([w[0] as u32, w[1] as u32], surf, x, mode),
            (32, 4) => sust_1d_b32_4(
                [w[0] as u32, w[1] as u32, w[2] as u32, w[3] as u32],
                surf,
                x,
                mode,
            ),
            (64, 1) => sust_1d_b64_1([w[0]], surf, x, mode),
            _ => sust_1d_b64_2([w[0], w[1]], surf, x, mode),
        }
    }
}

/// `surf1Dwrite<T>(v, surf, ...)`: [`surf1d_write_mode`] with CUDA's default [`Boundary::Trap`].
///
/// # Safety
///
/// As [`surf1d_read_mode`].
#[inline(always)]
pub unsafe fn surf1d_write<T: SurfElem>(v: T, surf: SurfObject, x: i32) {
    // SAFETY: forwarded contract.
    unsafe { surf1d_write_mode(v, surf, x, Boundary::Trap) }
}

/// `surf2Dread<T>(surf, ..., mode)`: PTX `suld.b.2d.{b8..v4.b32,b64,v2.b64}.{trap,clamp,zero}`.
///
/// # Safety
///
/// `surf` must be a live surface object over a 2D array.
/// Its element must have `T`'s size and `x` is in bytes (see [`SurfElem`]). With
/// [`Boundary::Trap`] an out-of-range coordinate aborts the launch.
#[inline(always)]
pub unsafe fn surf2d_read_mode<T: SurfElem>(surf: SurfObject, x: i32, y: i32, mode: Boundary) -> T {
    // SAFETY: forwarded contract; `T::BITS` and `T::N` are constants, so one arm survives.
    let w = unsafe {
        match (T::BITS, T::N) {
            (8, 1) => {
                let [a] = suld_2d_b8_1(surf, x, y, mode);
                [u64::from(a), 0, 0, 0]
            }
            (8, 2) => {
                let [a, b] = suld_2d_b8_2(surf, x, y, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (8, 4) => {
                let [a, b, c, d] = suld_2d_b8_4(surf, x, y, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (16, 1) => {
                let [a] = suld_2d_b16_1(surf, x, y, mode);
                [u64::from(a), 0, 0, 0]
            }
            (16, 2) => {
                let [a, b] = suld_2d_b16_2(surf, x, y, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (16, 4) => {
                let [a, b, c, d] = suld_2d_b16_4(surf, x, y, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (32, 1) => {
                let [a] = suld_2d_b32_1(surf, x, y, mode);
                [u64::from(a), 0, 0, 0]
            }
            (32, 2) => {
                let [a, b] = suld_2d_b32_2(surf, x, y, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (32, 4) => {
                let [a, b, c, d] = suld_2d_b32_4(surf, x, y, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (64, 1) => {
                let [a] = suld_2d_b64_1(surf, x, y, mode);
                [a, 0, 0, 0]
            }
            _ => {
                let [a, b] = suld_2d_b64_2(surf, x, y, mode);
                [a, b, 0, 0]
            }
        }
    };
    T::from_bits(w)
}

/// `surf2Dread<T>(surf, ...)`: [`surf2d_read_mode`] with CUDA's default [`Boundary::Trap`].
///
/// # Safety
///
/// As [`surf2d_read_mode`].
#[inline(always)]
pub unsafe fn surf2d_read<T: SurfElem>(surf: SurfObject, x: i32, y: i32) -> T {
    // SAFETY: forwarded contract.
    unsafe { surf2d_read_mode(surf, x, y, Boundary::Trap) }
}

/// `surf2Dwrite<T>(v, surf, ..., mode)`: PTX `sust.b.2d.{b8..v4.b32,b64,v2.b64}.{trap,clamp,zero}`.
///
/// # Safety
///
/// As [`surf2d_read_mode`].
#[inline(always)]
pub unsafe fn surf2d_write_mode<T: SurfElem>(
    v: T,
    surf: SurfObject,
    x: i32,
    y: i32,
    mode: Boundary,
) {
    let w = v.to_bits();
    // SAFETY: forwarded contract; `T::BITS` and `T::N` are constants, so one arm survives; the
    // narrowing casts keep exactly the component bits `to_bits` zero-extended.
    unsafe {
        match (T::BITS, T::N) {
            (8, 1) => sust_2d_b8_1([w[0] as u16], surf, x, y, mode),
            (8, 2) => sust_2d_b8_2([w[0] as u16, w[1] as u16], surf, x, y, mode),
            (8, 4) => sust_2d_b8_4(
                [w[0] as u16, w[1] as u16, w[2] as u16, w[3] as u16],
                surf,
                x,
                y,
                mode,
            ),
            (16, 1) => sust_2d_b16_1([w[0] as u16], surf, x, y, mode),
            (16, 2) => sust_2d_b16_2([w[0] as u16, w[1] as u16], surf, x, y, mode),
            (16, 4) => sust_2d_b16_4(
                [w[0] as u16, w[1] as u16, w[2] as u16, w[3] as u16],
                surf,
                x,
                y,
                mode,
            ),
            (32, 1) => sust_2d_b32_1([w[0] as u32], surf, x, y, mode),
            (32, 2) => sust_2d_b32_2([w[0] as u32, w[1] as u32], surf, x, y, mode),
            (32, 4) => sust_2d_b32_4(
                [w[0] as u32, w[1] as u32, w[2] as u32, w[3] as u32],
                surf,
                x,
                y,
                mode,
            ),
            (64, 1) => sust_2d_b64_1([w[0]], surf, x, y, mode),
            _ => sust_2d_b64_2([w[0], w[1]], surf, x, y, mode),
        }
    }
}

/// `surf2Dwrite<T>(v, surf, ...)`: [`surf2d_write_mode`] with CUDA's default [`Boundary::Trap`].
///
/// # Safety
///
/// As [`surf2d_read_mode`].
#[inline(always)]
pub unsafe fn surf2d_write<T: SurfElem>(v: T, surf: SurfObject, x: i32, y: i32) {
    // SAFETY: forwarded contract.
    unsafe { surf2d_write_mode(v, surf, x, y, Boundary::Trap) }
}

/// `surf3Dread<T>(surf, ..., mode)`: PTX `suld.b.3d.{b8..v4.b32,b64,v2.b64}.{trap,clamp,zero}`.
///
/// # Safety
///
/// `surf` must be a live surface object over a 3D array.
/// Its element must have `T`'s size and `x` is in bytes (see [`SurfElem`]). With
/// [`Boundary::Trap`] an out-of-range coordinate aborts the launch.
#[inline(always)]
pub unsafe fn surf3d_read_mode<T: SurfElem>(
    surf: SurfObject,
    x: i32,
    y: i32,
    z: i32,
    mode: Boundary,
) -> T {
    // SAFETY: forwarded contract; `T::BITS` and `T::N` are constants, so one arm survives.
    let w = unsafe {
        match (T::BITS, T::N) {
            (8, 1) => {
                let [a] = suld_3d_b8_1(surf, x, y, z, mode);
                [u64::from(a), 0, 0, 0]
            }
            (8, 2) => {
                let [a, b] = suld_3d_b8_2(surf, x, y, z, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (8, 4) => {
                let [a, b, c, d] = suld_3d_b8_4(surf, x, y, z, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (16, 1) => {
                let [a] = suld_3d_b16_1(surf, x, y, z, mode);
                [u64::from(a), 0, 0, 0]
            }
            (16, 2) => {
                let [a, b] = suld_3d_b16_2(surf, x, y, z, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (16, 4) => {
                let [a, b, c, d] = suld_3d_b16_4(surf, x, y, z, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (32, 1) => {
                let [a] = suld_3d_b32_1(surf, x, y, z, mode);
                [u64::from(a), 0, 0, 0]
            }
            (32, 2) => {
                let [a, b] = suld_3d_b32_2(surf, x, y, z, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (32, 4) => {
                let [a, b, c, d] = suld_3d_b32_4(surf, x, y, z, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (64, 1) => {
                let [a] = suld_3d_b64_1(surf, x, y, z, mode);
                [a, 0, 0, 0]
            }
            _ => {
                let [a, b] = suld_3d_b64_2(surf, x, y, z, mode);
                [a, b, 0, 0]
            }
        }
    };
    T::from_bits(w)
}

/// `surf3Dread<T>(surf, ...)`: [`surf3d_read_mode`] with CUDA's default [`Boundary::Trap`].
///
/// # Safety
///
/// As [`surf3d_read_mode`].
#[inline(always)]
pub unsafe fn surf3d_read<T: SurfElem>(surf: SurfObject, x: i32, y: i32, z: i32) -> T {
    // SAFETY: forwarded contract.
    unsafe { surf3d_read_mode(surf, x, y, z, Boundary::Trap) }
}

/// `surf3Dwrite<T>(v, surf, ..., mode)`: PTX `sust.b.3d.{b8..v4.b32,b64,v2.b64}.{trap,clamp,zero}`.
///
/// # Safety
///
/// As [`surf3d_read_mode`].
#[inline(always)]
pub unsafe fn surf3d_write_mode<T: SurfElem>(
    v: T,
    surf: SurfObject,
    x: i32,
    y: i32,
    z: i32,
    mode: Boundary,
) {
    let w = v.to_bits();
    // SAFETY: forwarded contract; `T::BITS` and `T::N` are constants, so one arm survives; the
    // narrowing casts keep exactly the component bits `to_bits` zero-extended.
    unsafe {
        match (T::BITS, T::N) {
            (8, 1) => sust_3d_b8_1([w[0] as u16], surf, x, y, z, mode),
            (8, 2) => sust_3d_b8_2([w[0] as u16, w[1] as u16], surf, x, y, z, mode),
            (8, 4) => sust_3d_b8_4(
                [w[0] as u16, w[1] as u16, w[2] as u16, w[3] as u16],
                surf,
                x,
                y,
                z,
                mode,
            ),
            (16, 1) => sust_3d_b16_1([w[0] as u16], surf, x, y, z, mode),
            (16, 2) => sust_3d_b16_2([w[0] as u16, w[1] as u16], surf, x, y, z, mode),
            (16, 4) => sust_3d_b16_4(
                [w[0] as u16, w[1] as u16, w[2] as u16, w[3] as u16],
                surf,
                x,
                y,
                z,
                mode,
            ),
            (32, 1) => sust_3d_b32_1([w[0] as u32], surf, x, y, z, mode),
            (32, 2) => sust_3d_b32_2([w[0] as u32, w[1] as u32], surf, x, y, z, mode),
            (32, 4) => sust_3d_b32_4(
                [w[0] as u32, w[1] as u32, w[2] as u32, w[3] as u32],
                surf,
                x,
                y,
                z,
                mode,
            ),
            (64, 1) => sust_3d_b64_1([w[0]], surf, x, y, z, mode),
            _ => sust_3d_b64_2([w[0], w[1]], surf, x, y, z, mode),
        }
    }
}

/// `surf3Dwrite<T>(v, surf, ...)`: [`surf3d_write_mode`] with CUDA's default [`Boundary::Trap`].
///
/// # Safety
///
/// As [`surf3d_read_mode`].
#[inline(always)]
pub unsafe fn surf3d_write<T: SurfElem>(v: T, surf: SurfObject, x: i32, y: i32, z: i32) {
    // SAFETY: forwarded contract.
    unsafe { surf3d_write_mode(v, surf, x, y, z, Boundary::Trap) }
}

/// `surf1DLayeredread<T>(surf, ..., mode)`: PTX `suld.b.a1d.{b8..v4.b32,b64,v2.b64}.{trap,clamp,zero}`.
///
/// # Safety
///
/// `surf` must be a live surface object over a 1D layered array.
/// Its element must have `T`'s size and `x` is in bytes (see [`SurfElem`]). With
/// [`Boundary::Trap`] an out-of-range coordinate aborts the launch.
#[inline(always)]
pub unsafe fn surf1d_layered_read_mode<T: SurfElem>(
    surf: SurfObject,
    x: i32,
    layer: u32,
    mode: Boundary,
) -> T {
    // SAFETY: forwarded contract; `T::BITS` and `T::N` are constants, so one arm survives.
    let w = unsafe {
        match (T::BITS, T::N) {
            (8, 1) => {
                let [a] = suld_a1d_b8_1(surf, x, layer, mode);
                [u64::from(a), 0, 0, 0]
            }
            (8, 2) => {
                let [a, b] = suld_a1d_b8_2(surf, x, layer, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (8, 4) => {
                let [a, b, c, d] = suld_a1d_b8_4(surf, x, layer, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (16, 1) => {
                let [a] = suld_a1d_b16_1(surf, x, layer, mode);
                [u64::from(a), 0, 0, 0]
            }
            (16, 2) => {
                let [a, b] = suld_a1d_b16_2(surf, x, layer, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (16, 4) => {
                let [a, b, c, d] = suld_a1d_b16_4(surf, x, layer, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (32, 1) => {
                let [a] = suld_a1d_b32_1(surf, x, layer, mode);
                [u64::from(a), 0, 0, 0]
            }
            (32, 2) => {
                let [a, b] = suld_a1d_b32_2(surf, x, layer, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (32, 4) => {
                let [a, b, c, d] = suld_a1d_b32_4(surf, x, layer, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (64, 1) => {
                let [a] = suld_a1d_b64_1(surf, x, layer, mode);
                [a, 0, 0, 0]
            }
            _ => {
                let [a, b] = suld_a1d_b64_2(surf, x, layer, mode);
                [a, b, 0, 0]
            }
        }
    };
    T::from_bits(w)
}

/// `surf1DLayeredread<T>(surf, ...)`: [`surf1d_layered_read_mode`] with CUDA's default [`Boundary::Trap`].
///
/// # Safety
///
/// As [`surf1d_layered_read_mode`].
#[inline(always)]
pub unsafe fn surf1d_layered_read<T: SurfElem>(surf: SurfObject, x: i32, layer: u32) -> T {
    // SAFETY: forwarded contract.
    unsafe { surf1d_layered_read_mode(surf, x, layer, Boundary::Trap) }
}

/// `surf1DLayeredwrite<T>(v, surf, ..., mode)`: PTX `sust.b.a1d.{b8..v4.b32,b64,v2.b64}.{trap,clamp,zero}`.
///
/// # Safety
///
/// As [`surf1d_layered_read_mode`].
#[inline(always)]
pub unsafe fn surf1d_layered_write_mode<T: SurfElem>(
    v: T,
    surf: SurfObject,
    x: i32,
    layer: u32,
    mode: Boundary,
) {
    let w = v.to_bits();
    // SAFETY: forwarded contract; `T::BITS` and `T::N` are constants, so one arm survives; the
    // narrowing casts keep exactly the component bits `to_bits` zero-extended.
    unsafe {
        match (T::BITS, T::N) {
            (8, 1) => sust_a1d_b8_1([w[0] as u16], surf, x, layer, mode),
            (8, 2) => sust_a1d_b8_2([w[0] as u16, w[1] as u16], surf, x, layer, mode),
            (8, 4) => sust_a1d_b8_4(
                [w[0] as u16, w[1] as u16, w[2] as u16, w[3] as u16],
                surf,
                x,
                layer,
                mode,
            ),
            (16, 1) => sust_a1d_b16_1([w[0] as u16], surf, x, layer, mode),
            (16, 2) => sust_a1d_b16_2([w[0] as u16, w[1] as u16], surf, x, layer, mode),
            (16, 4) => sust_a1d_b16_4(
                [w[0] as u16, w[1] as u16, w[2] as u16, w[3] as u16],
                surf,
                x,
                layer,
                mode,
            ),
            (32, 1) => sust_a1d_b32_1([w[0] as u32], surf, x, layer, mode),
            (32, 2) => sust_a1d_b32_2([w[0] as u32, w[1] as u32], surf, x, layer, mode),
            (32, 4) => sust_a1d_b32_4(
                [w[0] as u32, w[1] as u32, w[2] as u32, w[3] as u32],
                surf,
                x,
                layer,
                mode,
            ),
            (64, 1) => sust_a1d_b64_1([w[0]], surf, x, layer, mode),
            _ => sust_a1d_b64_2([w[0], w[1]], surf, x, layer, mode),
        }
    }
}

/// `surf1DLayeredwrite<T>(v, surf, ...)`: [`surf1d_layered_write_mode`] with CUDA's default [`Boundary::Trap`].
///
/// # Safety
///
/// As [`surf1d_layered_read_mode`].
#[inline(always)]
pub unsafe fn surf1d_layered_write<T: SurfElem>(v: T, surf: SurfObject, x: i32, layer: u32) {
    // SAFETY: forwarded contract.
    unsafe { surf1d_layered_write_mode(v, surf, x, layer, Boundary::Trap) }
}

/// `surf2DLayeredread<T>(surf, ..., mode)`: PTX `suld.b.a2d.{b8..v4.b32,b64,v2.b64}.{trap,clamp,zero}`.
///
/// # Safety
///
/// `surf` must be a live surface object over a 2D layered array.
/// Its element must have `T`'s size and `x` is in bytes (see [`SurfElem`]). With
/// [`Boundary::Trap`] an out-of-range coordinate aborts the launch.
#[inline(always)]
pub unsafe fn surf2d_layered_read_mode<T: SurfElem>(
    surf: SurfObject,
    x: i32,
    y: i32,
    layer: u32,
    mode: Boundary,
) -> T {
    // SAFETY: forwarded contract; `T::BITS` and `T::N` are constants, so one arm survives.
    let w = unsafe {
        match (T::BITS, T::N) {
            (8, 1) => {
                let [a] = suld_a2d_b8_1(surf, x, y, layer, mode);
                [u64::from(a), 0, 0, 0]
            }
            (8, 2) => {
                let [a, b] = suld_a2d_b8_2(surf, x, y, layer, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (8, 4) => {
                let [a, b, c, d] = suld_a2d_b8_4(surf, x, y, layer, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (16, 1) => {
                let [a] = suld_a2d_b16_1(surf, x, y, layer, mode);
                [u64::from(a), 0, 0, 0]
            }
            (16, 2) => {
                let [a, b] = suld_a2d_b16_2(surf, x, y, layer, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (16, 4) => {
                let [a, b, c, d] = suld_a2d_b16_4(surf, x, y, layer, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (32, 1) => {
                let [a] = suld_a2d_b32_1(surf, x, y, layer, mode);
                [u64::from(a), 0, 0, 0]
            }
            (32, 2) => {
                let [a, b] = suld_a2d_b32_2(surf, x, y, layer, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (32, 4) => {
                let [a, b, c, d] = suld_a2d_b32_4(surf, x, y, layer, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (64, 1) => {
                let [a] = suld_a2d_b64_1(surf, x, y, layer, mode);
                [a, 0, 0, 0]
            }
            _ => {
                let [a, b] = suld_a2d_b64_2(surf, x, y, layer, mode);
                [a, b, 0, 0]
            }
        }
    };
    T::from_bits(w)
}

/// `surf2DLayeredread<T>(surf, ...)`: [`surf2d_layered_read_mode`] with CUDA's default [`Boundary::Trap`].
///
/// # Safety
///
/// As [`surf2d_layered_read_mode`].
#[inline(always)]
pub unsafe fn surf2d_layered_read<T: SurfElem>(surf: SurfObject, x: i32, y: i32, layer: u32) -> T {
    // SAFETY: forwarded contract.
    unsafe { surf2d_layered_read_mode(surf, x, y, layer, Boundary::Trap) }
}

/// `surf2DLayeredwrite<T>(v, surf, ..., mode)`: PTX `sust.b.a2d.{b8..v4.b32,b64,v2.b64}.{trap,clamp,zero}`.
///
/// # Safety
///
/// As [`surf2d_layered_read_mode`].
#[inline(always)]
pub unsafe fn surf2d_layered_write_mode<T: SurfElem>(
    v: T,
    surf: SurfObject,
    x: i32,
    y: i32,
    layer: u32,
    mode: Boundary,
) {
    let w = v.to_bits();
    // SAFETY: forwarded contract; `T::BITS` and `T::N` are constants, so one arm survives; the
    // narrowing casts keep exactly the component bits `to_bits` zero-extended.
    unsafe {
        match (T::BITS, T::N) {
            (8, 1) => sust_a2d_b8_1([w[0] as u16], surf, x, y, layer, mode),
            (8, 2) => sust_a2d_b8_2([w[0] as u16, w[1] as u16], surf, x, y, layer, mode),
            (8, 4) => sust_a2d_b8_4(
                [w[0] as u16, w[1] as u16, w[2] as u16, w[3] as u16],
                surf,
                x,
                y,
                layer,
                mode,
            ),
            (16, 1) => sust_a2d_b16_1([w[0] as u16], surf, x, y, layer, mode),
            (16, 2) => sust_a2d_b16_2([w[0] as u16, w[1] as u16], surf, x, y, layer, mode),
            (16, 4) => sust_a2d_b16_4(
                [w[0] as u16, w[1] as u16, w[2] as u16, w[3] as u16],
                surf,
                x,
                y,
                layer,
                mode,
            ),
            (32, 1) => sust_a2d_b32_1([w[0] as u32], surf, x, y, layer, mode),
            (32, 2) => sust_a2d_b32_2([w[0] as u32, w[1] as u32], surf, x, y, layer, mode),
            (32, 4) => sust_a2d_b32_4(
                [w[0] as u32, w[1] as u32, w[2] as u32, w[3] as u32],
                surf,
                x,
                y,
                layer,
                mode,
            ),
            (64, 1) => sust_a2d_b64_1([w[0]], surf, x, y, layer, mode),
            _ => sust_a2d_b64_2([w[0], w[1]], surf, x, y, layer, mode),
        }
    }
}

/// `surf2DLayeredwrite<T>(v, surf, ...)`: [`surf2d_layered_write_mode`] with CUDA's default [`Boundary::Trap`].
///
/// # Safety
///
/// As [`surf2d_layered_read_mode`].
#[inline(always)]
pub unsafe fn surf2d_layered_write<T: SurfElem>(
    v: T,
    surf: SurfObject,
    x: i32,
    y: i32,
    layer: u32,
) {
    // SAFETY: forwarded contract.
    unsafe { surf2d_layered_write_mode(v, surf, x, y, layer, Boundary::Trap) }
}

/// `surfCubemapread<T>(surf, ..., mode)`: PTX `suld.b.a2d.{b8..v4.b32,b64,v2.b64}.{trap,clamp,zero}`.
///
/// # Safety
///
/// `surf` must be a live surface object over a cubemap (`face` in `0..6`) array.
/// Its element must have `T`'s size and `x` is in bytes (see [`SurfElem`]). With
/// [`Boundary::Trap`] an out-of-range coordinate aborts the launch.
#[inline(always)]
pub unsafe fn surf_cube_read_mode<T: SurfElem>(
    surf: SurfObject,
    x: i32,
    y: i32,
    face: u32,
    mode: Boundary,
) -> T {
    // SAFETY: forwarded contract; `T::BITS` and `T::N` are constants, so one arm survives.
    let w = unsafe {
        match (T::BITS, T::N) {
            (8, 1) => {
                let [a] = suld_a2d_b8_1(surf, x, y, face, mode);
                [u64::from(a), 0, 0, 0]
            }
            (8, 2) => {
                let [a, b] = suld_a2d_b8_2(surf, x, y, face, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (8, 4) => {
                let [a, b, c, d] = suld_a2d_b8_4(surf, x, y, face, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (16, 1) => {
                let [a] = suld_a2d_b16_1(surf, x, y, face, mode);
                [u64::from(a), 0, 0, 0]
            }
            (16, 2) => {
                let [a, b] = suld_a2d_b16_2(surf, x, y, face, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (16, 4) => {
                let [a, b, c, d] = suld_a2d_b16_4(surf, x, y, face, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (32, 1) => {
                let [a] = suld_a2d_b32_1(surf, x, y, face, mode);
                [u64::from(a), 0, 0, 0]
            }
            (32, 2) => {
                let [a, b] = suld_a2d_b32_2(surf, x, y, face, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (32, 4) => {
                let [a, b, c, d] = suld_a2d_b32_4(surf, x, y, face, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (64, 1) => {
                let [a] = suld_a2d_b64_1(surf, x, y, face, mode);
                [a, 0, 0, 0]
            }
            _ => {
                let [a, b] = suld_a2d_b64_2(surf, x, y, face, mode);
                [a, b, 0, 0]
            }
        }
    };
    T::from_bits(w)
}

/// `surfCubemapread<T>(surf, ...)`: [`surf_cube_read_mode`] with CUDA's default [`Boundary::Trap`].
///
/// # Safety
///
/// As [`surf_cube_read_mode`].
#[inline(always)]
pub unsafe fn surf_cube_read<T: SurfElem>(surf: SurfObject, x: i32, y: i32, face: u32) -> T {
    // SAFETY: forwarded contract.
    unsafe { surf_cube_read_mode(surf, x, y, face, Boundary::Trap) }
}

/// `surfCubemapwrite<T>(v, surf, ..., mode)`: PTX `sust.b.a2d.{b8..v4.b32,b64,v2.b64}.{trap,clamp,zero}`.
///
/// # Safety
///
/// As [`surf_cube_read_mode`].
#[inline(always)]
pub unsafe fn surf_cube_write_mode<T: SurfElem>(
    v: T,
    surf: SurfObject,
    x: i32,
    y: i32,
    face: u32,
    mode: Boundary,
) {
    let w = v.to_bits();
    // SAFETY: forwarded contract; `T::BITS` and `T::N` are constants, so one arm survives; the
    // narrowing casts keep exactly the component bits `to_bits` zero-extended.
    unsafe {
        match (T::BITS, T::N) {
            (8, 1) => sust_a2d_b8_1([w[0] as u16], surf, x, y, face, mode),
            (8, 2) => sust_a2d_b8_2([w[0] as u16, w[1] as u16], surf, x, y, face, mode),
            (8, 4) => sust_a2d_b8_4(
                [w[0] as u16, w[1] as u16, w[2] as u16, w[3] as u16],
                surf,
                x,
                y,
                face,
                mode,
            ),
            (16, 1) => sust_a2d_b16_1([w[0] as u16], surf, x, y, face, mode),
            (16, 2) => sust_a2d_b16_2([w[0] as u16, w[1] as u16], surf, x, y, face, mode),
            (16, 4) => sust_a2d_b16_4(
                [w[0] as u16, w[1] as u16, w[2] as u16, w[3] as u16],
                surf,
                x,
                y,
                face,
                mode,
            ),
            (32, 1) => sust_a2d_b32_1([w[0] as u32], surf, x, y, face, mode),
            (32, 2) => sust_a2d_b32_2([w[0] as u32, w[1] as u32], surf, x, y, face, mode),
            (32, 4) => sust_a2d_b32_4(
                [w[0] as u32, w[1] as u32, w[2] as u32, w[3] as u32],
                surf,
                x,
                y,
                face,
                mode,
            ),
            (64, 1) => sust_a2d_b64_1([w[0]], surf, x, y, face, mode),
            _ => sust_a2d_b64_2([w[0], w[1]], surf, x, y, face, mode),
        }
    }
}

/// `surfCubemapwrite<T>(v, surf, ...)`: [`surf_cube_write_mode`] with CUDA's default [`Boundary::Trap`].
///
/// # Safety
///
/// As [`surf_cube_read_mode`].
#[inline(always)]
pub unsafe fn surf_cube_write<T: SurfElem>(v: T, surf: SurfObject, x: i32, y: i32, face: u32) {
    // SAFETY: forwarded contract.
    unsafe { surf_cube_write_mode(v, surf, x, y, face, Boundary::Trap) }
}

/// `surfCubemapLayeredread<T>(surf, ..., mode)`: PTX `suld.b.a2d.{b8..v4.b32,b64,v2.b64}.{trap,clamp,zero}`.
///
/// # Safety
///
/// `surf` must be a live surface object over a cubemap layered (`layer_face` = layer * 6 + face) array.
/// Its element must have `T`'s size and `x` is in bytes (see [`SurfElem`]). With
/// [`Boundary::Trap`] an out-of-range coordinate aborts the launch.
#[inline(always)]
pub unsafe fn surf_cube_layered_read_mode<T: SurfElem>(
    surf: SurfObject,
    x: i32,
    y: i32,
    layer_face: u32,
    mode: Boundary,
) -> T {
    // SAFETY: forwarded contract; `T::BITS` and `T::N` are constants, so one arm survives.
    let w = unsafe {
        match (T::BITS, T::N) {
            (8, 1) => {
                let [a] = suld_a2d_b8_1(surf, x, y, layer_face, mode);
                [u64::from(a), 0, 0, 0]
            }
            (8, 2) => {
                let [a, b] = suld_a2d_b8_2(surf, x, y, layer_face, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (8, 4) => {
                let [a, b, c, d] = suld_a2d_b8_4(surf, x, y, layer_face, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (16, 1) => {
                let [a] = suld_a2d_b16_1(surf, x, y, layer_face, mode);
                [u64::from(a), 0, 0, 0]
            }
            (16, 2) => {
                let [a, b] = suld_a2d_b16_2(surf, x, y, layer_face, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (16, 4) => {
                let [a, b, c, d] = suld_a2d_b16_4(surf, x, y, layer_face, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (32, 1) => {
                let [a] = suld_a2d_b32_1(surf, x, y, layer_face, mode);
                [u64::from(a), 0, 0, 0]
            }
            (32, 2) => {
                let [a, b] = suld_a2d_b32_2(surf, x, y, layer_face, mode);
                [u64::from(a), u64::from(b), 0, 0]
            }
            (32, 4) => {
                let [a, b, c, d] = suld_a2d_b32_4(surf, x, y, layer_face, mode);
                [u64::from(a), u64::from(b), u64::from(c), u64::from(d)]
            }
            (64, 1) => {
                let [a] = suld_a2d_b64_1(surf, x, y, layer_face, mode);
                [a, 0, 0, 0]
            }
            _ => {
                let [a, b] = suld_a2d_b64_2(surf, x, y, layer_face, mode);
                [a, b, 0, 0]
            }
        }
    };
    T::from_bits(w)
}

/// `surfCubemapLayeredread<T>(surf, ...)`: [`surf_cube_layered_read_mode`] with CUDA's default [`Boundary::Trap`].
///
/// # Safety
///
/// As [`surf_cube_layered_read_mode`].
#[inline(always)]
pub unsafe fn surf_cube_layered_read<T: SurfElem>(
    surf: SurfObject,
    x: i32,
    y: i32,
    layer_face: u32,
) -> T {
    // SAFETY: forwarded contract.
    unsafe { surf_cube_layered_read_mode(surf, x, y, layer_face, Boundary::Trap) }
}

/// `surfCubemapLayeredwrite<T>(v, surf, ..., mode)`: PTX `sust.b.a2d.{b8..v4.b32,b64,v2.b64}.{trap,clamp,zero}`.
///
/// # Safety
///
/// As [`surf_cube_layered_read_mode`].
#[inline(always)]
pub unsafe fn surf_cube_layered_write_mode<T: SurfElem>(
    v: T,
    surf: SurfObject,
    x: i32,
    y: i32,
    layer_face: u32,
    mode: Boundary,
) {
    let w = v.to_bits();
    // SAFETY: forwarded contract; `T::BITS` and `T::N` are constants, so one arm survives; the
    // narrowing casts keep exactly the component bits `to_bits` zero-extended.
    unsafe {
        match (T::BITS, T::N) {
            (8, 1) => sust_a2d_b8_1([w[0] as u16], surf, x, y, layer_face, mode),
            (8, 2) => sust_a2d_b8_2([w[0] as u16, w[1] as u16], surf, x, y, layer_face, mode),
            (8, 4) => sust_a2d_b8_4(
                [w[0] as u16, w[1] as u16, w[2] as u16, w[3] as u16],
                surf,
                x,
                y,
                layer_face,
                mode,
            ),
            (16, 1) => sust_a2d_b16_1([w[0] as u16], surf, x, y, layer_face, mode),
            (16, 2) => sust_a2d_b16_2([w[0] as u16, w[1] as u16], surf, x, y, layer_face, mode),
            (16, 4) => sust_a2d_b16_4(
                [w[0] as u16, w[1] as u16, w[2] as u16, w[3] as u16],
                surf,
                x,
                y,
                layer_face,
                mode,
            ),
            (32, 1) => sust_a2d_b32_1([w[0] as u32], surf, x, y, layer_face, mode),
            (32, 2) => sust_a2d_b32_2([w[0] as u32, w[1] as u32], surf, x, y, layer_face, mode),
            (32, 4) => sust_a2d_b32_4(
                [w[0] as u32, w[1] as u32, w[2] as u32, w[3] as u32],
                surf,
                x,
                y,
                layer_face,
                mode,
            ),
            (64, 1) => sust_a2d_b64_1([w[0]], surf, x, y, layer_face, mode),
            _ => sust_a2d_b64_2([w[0], w[1]], surf, x, y, layer_face, mode),
        }
    }
}

/// `surfCubemapLayeredwrite<T>(v, surf, ...)`: [`surf_cube_layered_write_mode`] with CUDA's default [`Boundary::Trap`].
///
/// # Safety
///
/// As [`surf_cube_layered_read_mode`].
#[inline(always)]
pub unsafe fn surf_cube_layered_write<T: SurfElem>(
    v: T,
    surf: SurfObject,
    x: i32,
    y: i32,
    layer_face: u32,
) {
    // SAFETY: forwarded contract.
    unsafe { surf_cube_layered_write_mode(v, surf, x, y, layer_face, Boundary::Trap) }
}
