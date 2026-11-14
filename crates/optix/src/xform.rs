//! Transform traversables: [`Transform`] (static, matrix motion, SRT motion) and
//! [`MotionOptions`].
//!
//! **What.** A transform is a small device-memory object OptiX traverses between an instance
//! structure and its child: a static 3x4 matrix (`OptixStaticTransform`), N >= 2 matrix keys
//! (`OptixMatrixMotionTransform`) or N >= 2 scale/rotation/translation keys
//! (`OptixSRTMotionTransform`, [`Srt`]) interpolated over [`MotionOptions`]' time range. It is
//! built from Rust data, uploaded once, and turned into a traversable handle with
//! `optixConvertPointerToTraversableHandle`.
//! **How.** The bytes are laid out at the offsets of the generated `sys` structs (checked at
//! compile time), with N keys in place of the header's fixed two, as `optix_types.h` describes;
//! padding is zero. The buffer comes from the synchronous allocator, whose alignment is checked
//! against `OPTIX_TRANSFORM_BYTE_ALIGNMENT` before use.
//! **Lifetimes.** A transform holds its [`Child`] (an [`Accel`] or another transform), so the
//! handle it embeds stays valid; transforms are immutable, so chains cannot form cycles.
//! Pipelines tracing motion transforms need `PipelineOptions::motion_blur` and a traversable
//! graph depth that counts each transform as one level.

use crate::{Accel, DeviceContext, Error, Result, check, entry, sys};
use cuda_core::{DeviceBuffer, Stream};
use std::mem::offset_of;
use std::sync::Arc;

/// Time range and border behaviour of a motion transform or motion structure
/// (`OptixMotionOptions` without the key count, which comes from the keys given).
///
/// A motion GAS with a vanish flag must be traced through an instance structure: traced as the
/// top-level handle, a ray outside the time range faults the launch with an illegal address
/// (observed: OptiX 9.1 driver, sm_86, single-GAS and any-graph pipelines alike).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionOptions {
    /// Time of the first key.
    pub begin: f32,
    /// Time of the last key (greater than `begin`).
    pub end: f32,
    /// Rays with time before `begin` miss the child (else they see the first key).
    pub start_vanish: bool,
    /// Rays with time after `end` miss the child (else they see the last key).
    pub end_vanish: bool,
}

impl Default for MotionOptions {
    /// Keys over `[0, 1]`, clamped at both ends.
    fn default() -> Self {
        Self {
            begin: 0.0,
            end: 1.0,
            start_vanish: false,
            end_vanish: false,
        }
    }
}

impl MotionOptions {
    /// The raw options for `keys` motion keys (1 = no motion; then the times are ignored).
    pub fn to_sys(&self, keys: u16) -> Result<sys::OptixMotionOptions> {
        if keys == 0 {
            return Err(Error::Invalid("motion needs at least one key"));
        }
        if keys > 1 && !(self.begin.is_finite() && self.end.is_finite() && self.begin < self.end) {
            return Err(Error::Invalid(
                "motion time range must be finite with begin < end",
            ));
        }
        let mut flags = sys::OptixMotionFlags_OPTIX_MOTION_FLAG_NONE;
        if self.start_vanish {
            flags |= sys::OptixMotionFlags_OPTIX_MOTION_FLAG_START_VANISH;
        }
        if self.end_vanish {
            flags |= sys::OptixMotionFlags_OPTIX_MOTION_FLAG_END_VANISH;
        }
        // Both flags fit in the low two bits.
        Ok(sys::OptixMotionOptions {
            numKeys: keys,
            flags: flags as u16,
            timeBegin: self.begin,
            timeEnd: self.end,
        })
    }
}

/// One SRT motion key (`OptixSRTData`): object-to-world is `T * R * S` where
/// `S = [[sx, a, b, pvx], [0, sy, c, pvy], [0, 0, sz, pvz]]` (scale, shear, pivot), `R` the unit
/// quaternion `(qx, qy, qz, qw)` and `T` the translation `(tx, ty, tz)`; components are
/// interpolated linearly between keys.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Srt {
    /// Scale along x.
    pub sx: f32,
    /// Shear of y into x.
    pub a: f32,
    /// Shear of z into x.
    pub b: f32,
    /// Pivot x (translation applied before the rotation).
    pub pvx: f32,
    /// Scale along y.
    pub sy: f32,
    /// Shear of z into y.
    pub c: f32,
    /// Pivot y.
    pub pvy: f32,
    /// Scale along z.
    pub sz: f32,
    /// Pivot z.
    pub pvz: f32,
    /// Rotation quaternion x (`sin(theta/2) * axis.x`).
    pub qx: f32,
    /// Rotation quaternion y.
    pub qy: f32,
    /// Rotation quaternion z.
    pub qz: f32,
    /// Rotation quaternion w (`cos(theta/2)`).
    pub qw: f32,
    /// Translation x (after the rotation).
    pub tx: f32,
    /// Translation y.
    pub ty: f32,
    /// Translation z.
    pub tz: f32,
}

impl Srt {
    /// No scale, rotation or translation.
    pub const IDENTITY: Self = Self {
        sx: 1.0,
        a: 0.0,
        b: 0.0,
        pvx: 0.0,
        sy: 1.0,
        c: 0.0,
        pvy: 0.0,
        sz: 1.0,
        pvz: 0.0,
        qx: 0.0,
        qy: 0.0,
        qz: 0.0,
        qw: 1.0,
        tx: 0.0,
        ty: 0.0,
        tz: 0.0,
    };

    /// The 16 floats in `OptixSRTData` order.
    fn words(&self) -> [f32; 16] {
        let s = self;
        [
            s.sx, s.a, s.b, s.pvx, s.sy, s.c, s.pvy, s.sz, s.pvz, s.qx, s.qy, s.qz, s.qw, s.tx,
            s.ty, s.tz,
        ]
    }
}

// The SRT key and the motion options are written field by field at these offsets.
const _: () = assert!(size_of::<sys::OptixSRTData>() == 16 * size_of::<f32>());
const _: () = assert!(offset_of!(sys::OptixSRTData, tz) == 15 * size_of::<f32>());
const _: () =
    assert!(size_of::<sys::OptixMotionOptions>() == 2 * size_of::<u16>() + 2 * size_of::<f32>());
// Two keys of the variable-length layouts make exactly the fixed-size header structs.
const _: () = assert!(
    offset_of!(sys::OptixMatrixMotionTransform, transform) + 2 * size_of::<[f32; 12]>()
        == size_of::<sys::OptixMatrixMotionTransform>()
);
const _: () = assert!(
    offset_of!(sys::OptixSRTMotionTransform, srtData) + 2 * size_of::<sys::OptixSRTData>()
        == size_of::<sys::OptixSRTMotionTransform>()
);

/// What a transform transforms.
#[derive(Clone)]
pub enum Child {
    /// A geometry or instance structure.
    Accel(Arc<Accel>),
    /// Another transform.
    Transform(Arc<Transform>),
    /// One GAS of a cluster build ([`crate::ClusterAccel::child`]): the build and the GAS handle.
    Cluster(Arc<crate::ClusterAccel>, sys::OptixTraversableHandle),
}

impl Child {
    /// The child's traversable handle.
    pub fn handle(&self) -> sys::OptixTraversableHandle {
        match self {
            Self::Accel(a) => a.handle(),
            Self::Transform(t) => t.handle(),
            Self::Cluster(_, h) => *h,
        }
    }

    /// The device context the child lives in.
    pub fn context(&self) -> &Arc<DeviceContext> {
        match self {
            Self::Accel(a) => a.context(),
            Self::Transform(t) => &t.ctx,
            Self::Cluster(c, _) => c.context(),
        }
    }
}

impl From<Arc<Accel>> for Child {
    fn from(a: Arc<Accel>) -> Self {
        Self::Accel(a)
    }
}

impl From<Arc<Transform>> for Child {
    fn from(t: Arc<Transform>) -> Self {
        Self::Transform(t)
    }
}

impl From<&Arc<Accel>> for Child {
    fn from(a: &Arc<Accel>) -> Self {
        Self::Accel(a.clone())
    }
}

impl From<&Arc<Transform>> for Child {
    fn from(t: &Arc<Transform>) -> Self {
        Self::Transform(t.clone())
    }
}

/// Which transform a [`Transform`] is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransformKind {
    /// `OPTIX_TRAVERSABLE_TYPE_STATIC_TRANSFORM`.
    Static,
    /// `OPTIX_TRAVERSABLE_TYPE_MATRIX_MOTION_TRANSFORM`.
    MatrixMotion,
    /// `OPTIX_TRAVERSABLE_TYPE_SRT_MOTION_TRANSFORM`.
    SrtMotion,
}

impl TransformKind {
    fn to_sys(self) -> sys::OptixTraversableType {
        match self {
            Self::Static => sys::OptixTraversableType_OPTIX_TRAVERSABLE_TYPE_STATIC_TRANSFORM,
            Self::MatrixMotion => {
                sys::OptixTraversableType_OPTIX_TRAVERSABLE_TYPE_MATRIX_MOTION_TRANSFORM
            }
            Self::SrtMotion => {
                sys::OptixTraversableType_OPTIX_TRAVERSABLE_TYPE_SRT_MOTION_TRANSFORM
            }
        }
    }
}

/// Zero-initialised transform bytes, written at checked offsets.
struct Bytes(Vec<u8>);

impl Bytes {
    fn put(&mut self, at: usize, data: &[u8]) -> Result<()> {
        let end = at
            .checked_add(data.len())
            .ok_or(Error::Invalid("transform layout overflows"))?;
        let dst = self
            .0
            .get_mut(at..end)
            .ok_or(Error::Invalid("transform layout exceeds its buffer"))?;
        dst.copy_from_slice(data);
        Ok(())
    }

    fn floats(&mut self, at: usize, v: &[f32]) -> Result<()> {
        for (i, f) in v.iter().enumerate() {
            self.put(at + i * size_of::<f32>(), &f.to_ne_bytes())?;
        }
        Ok(())
    }

    fn motion(&mut self, at: usize, m: sys::OptixMotionOptions) -> Result<()> {
        type M = sys::OptixMotionOptions;
        self.put(at + offset_of!(M, numKeys), &m.numKeys.to_ne_bytes())?;
        self.put(at + offset_of!(M, flags), &m.flags.to_ne_bytes())?;
        self.put(at + offset_of!(M, timeBegin), &m.timeBegin.to_ne_bytes())?;
        self.put(at + offset_of!(M, timeEnd), &m.timeEnd.to_ne_bytes())
    }
}

/// Inverse of a row-major 3x4 affine matrix (computed in `f64`); a singular or non-finite
/// matrix is an error.
fn invert(m: &[f32; 12]) -> Result<[f32; 12]> {
    let g = |i: usize| f64::from(m[i]);
    let (a, b, c, d, e, f, h, i, j) = (g(0), g(1), g(2), g(4), g(5), g(6), g(8), g(9), g(10));
    let (tx, ty, tz) = (g(3), g(7), g(11));
    let co = [e * j - f * i, f * h - d * j, d * i - e * h];
    let det = a * co[0] + b * co[1] + c * co[2];
    if !det.is_finite() || det == 0.0 {
        return Err(Error::Invalid("transform matrix is singular or not finite"));
    }
    let r = 1.0 / det;
    let inv3 = [
        [co[0] * r, (c * i - b * j) * r, (b * f - c * e) * r],
        [co[1] * r, (a * j - c * h) * r, (c * d - a * f) * r],
        [co[2] * r, (b * h - a * i) * r, (a * e - b * d) * r],
    ];
    let mut out = [0f32; 12];
    for (row, r3) in inv3.iter().enumerate() {
        let t = -(r3[0] * tx + r3[1] * ty + r3[2] * tz);
        let vals = [r3[0], r3[1], r3[2], t];
        for (col, v) in vals.iter().enumerate() {
            out[row * 4 + col] = *v as f32;
        }
    }
    if out.iter().any(|v| !v.is_finite()) {
        return Err(Error::Invalid("transform inverse is not finite in f32"));
    }
    Ok(out)
}

/// A transform traversable on the device.
pub struct Transform {
    handle: sys::OptixTraversableHandle,
    kind: TransformKind,
    buffer: DeviceBuffer<u8>,
    child: Child,
    ctx: Arc<DeviceContext>,
}

impl Transform {
    /// A static transform of `child` by the row-major 3x4 object-to-world matrix `xform` (its
    /// inverse is computed). Uploads on `stream` and synchronizes it.
    pub fn fixed(
        ctx: &Arc<DeviceContext>,
        stream: &Stream,
        child: impl Into<Child>,
        xform: [f32; 12],
    ) -> Result<Arc<Self>> {
        let child = child.into();
        let inv = invert(&xform)?;
        let mut b = Bytes(vec![0; size_of::<sys::OptixStaticTransform>()]);
        b.put(
            offset_of!(sys::OptixStaticTransform, child),
            &child.handle().to_ne_bytes(),
        )?;
        b.floats(offset_of!(sys::OptixStaticTransform, transform), &xform)?;
        b.floats(offset_of!(sys::OptixStaticTransform, invTransform), &inv)?;
        Self::upload(ctx, stream, child, TransformKind::Static, b)
    }

    /// A matrix motion transform of `child`: `keys` (at least two) row-major 3x4
    /// object-to-world matrices spread evenly over `motion`'s time range. Uploads on `stream`
    /// and synchronizes it.
    pub fn matrix_motion(
        ctx: &Arc<DeviceContext>,
        stream: &Stream,
        child: impl Into<Child>,
        motion: MotionOptions,
        keys: &[[f32; 12]],
    ) -> Result<Arc<Self>> {
        let child = child.into();
        let (opts, start) = Self::motion_header(
            &motion,
            keys.len(),
            offset_of!(sys::OptixMatrixMotionTransform, transform),
        )?;
        let key = size_of::<[f32; 12]>();
        let mut b = Bytes(vec![0; start + size_of_val(keys)]);
        b.put(
            offset_of!(sys::OptixMatrixMotionTransform, child),
            &child.handle().to_ne_bytes(),
        )?;
        b.motion(
            offset_of!(sys::OptixMatrixMotionTransform, motionOptions),
            opts,
        )?;
        for (i, k) in keys.iter().enumerate() {
            b.floats(start + i * key, k)?;
        }
        Self::upload(ctx, stream, child, TransformKind::MatrixMotion, b)
    }

    /// An SRT motion transform of `child`: `keys` (at least two) spread evenly over `motion`'s
    /// time range. Uploads on `stream` and synchronizes it.
    pub fn srt_motion(
        ctx: &Arc<DeviceContext>,
        stream: &Stream,
        child: impl Into<Child>,
        motion: MotionOptions,
        keys: &[Srt],
    ) -> Result<Arc<Self>> {
        let child = child.into();
        let (opts, start) = Self::motion_header(
            &motion,
            keys.len(),
            offset_of!(sys::OptixSRTMotionTransform, srtData),
        )?;
        let key = size_of::<sys::OptixSRTData>();
        let mut b = Bytes(vec![0; start + keys.len() * key]);
        b.put(
            offset_of!(sys::OptixSRTMotionTransform, child),
            &child.handle().to_ne_bytes(),
        )?;
        b.motion(
            offset_of!(sys::OptixSRTMotionTransform, motionOptions),
            opts,
        )?;
        for (i, k) in keys.iter().enumerate() {
            b.floats(start + i * key, &k.words())?;
        }
        Self::upload(ctx, stream, child, TransformKind::SrtMotion, b)
    }

    /// Validated motion options for `n` keys and the byte offset of the first key.
    fn motion_header(
        motion: &MotionOptions,
        n: usize,
        start: usize,
    ) -> Result<(sys::OptixMotionOptions, usize)> {
        let keys = u16::try_from(n).map_err(|_| Error::Invalid("too many motion keys"))?;
        if keys < 2 {
            return Err(Error::Invalid("a motion transform needs at least two keys"));
        }
        Ok((motion.to_sys(keys)?, start))
    }

    fn upload(
        ctx: &Arc<DeviceContext>,
        stream: &Stream,
        child: Child,
        kind: TransformKind,
        bytes: Bytes,
    ) -> Result<Arc<Self>> {
        if !Arc::ptr_eq(child.context(), ctx) {
            return Err(Error::Invalid(
                "transform child from another device context",
            ));
        }
        if stream.context().cu_ctx() != ctx.cuda().cu_ctx() {
            return Err(Error::Invalid(
                "stream belongs to another CUDA context than the device context",
            ));
        }
        let buffer = DeviceBuffer::from_host(stream, &bytes.0)?;
        if buffer.cu_deviceptr() % u64::from(sys::OPTIX_TRANSFORM_BYTE_ALIGNMENT) != 0 {
            return Err(Error::Invalid(
                "allocator returned a transform buffer below OptiX's 64-byte alignment",
            ));
        }
        let mut handle = 0;
        ctx.cuda().bind_to_thread()?;
        // SAFETY: `buffer` holds a complete, aligned transform of `kind` on this context (written
        // and synchronized by `from_host`); `handle` is host memory.
        check("optixConvertPointerToTraversableHandle", unsafe {
            entry!(ctx.table(), optixConvertPointerToTraversableHandle)(
                ctx.raw(),
                buffer.cu_deviceptr(),
                kind.to_sys(),
                &mut handle,
            )
        })?;
        Ok(Arc::new(Self {
            handle,
            kind,
            buffer,
            child,
            ctx: ctx.clone(),
        }))
    }

    /// The handle to trace against or to reference from an instance.
    pub fn handle(&self) -> sys::OptixTraversableHandle {
        self.handle
    }

    /// Which transform this is.
    pub fn kind(&self) -> TransformKind {
        self.kind
    }

    /// What it transforms.
    pub fn child(&self) -> &Child {
        &self.child
    }

    /// Bytes on the device.
    pub fn size_bytes(&self) -> usize {
        self.buffer.len()
    }

    /// The device context it was created on.
    pub fn context(&self) -> &Arc<DeviceContext> {
        &self.ctx
    }
}
