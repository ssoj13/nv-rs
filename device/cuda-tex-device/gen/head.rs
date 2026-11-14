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
//! crate `cuda-core-ext` (`TextureObject::handle`, `SurfaceObject::handle`).

#![no_std]

use cuda_device::ptx_asm;
use sealed::{Scalar, SurfBits, TexBits};

/// A texture object handle (`cudaTextureObject_t`), from `cuda_core_ext::TextureObject::handle`.
pub type TexObject = u64;

/// A surface object handle (`cudaSurfaceObject_t`), from `cuda_core_ext::SurfaceObject::handle`.
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
@@TRAIT@@
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

@@SURFELEMS@@

@@IMPLS@@

@@TEXFNS@@

@@GATHER@@

@@SURFRAW@@
@@PUBSURF@@