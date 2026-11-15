//! The panic-free clamp of the camera crates.
//!
//! `f32::clamp` panics when `lo > hi` or a bound is NaN, and the bounds here (pitch limit,
//! distance range) come from host settings. camera-core is pure (no UI dependency), so it keeps its
//! own copy of the rule. Used by `projection`, `dynamics`, `orientation`, `rig` and the controllers
//! wherever a bound is not a literal constant.

/// `v` limited to `[lo, hi]` without panicking: an inverted range collapses to `lo`, a NaN `v`
/// becomes `lo`, a NaN `lo` is ignored and a NaN `hi` collapses to `lo` (the `f32::max` / `min`
/// rule).
pub(crate) fn clamp(v: f32, lo: f32, hi: f32) -> f32 {
    if v.is_nan() {
        lo
    } else {
        v.max(lo).min(hi.max(lo))
    }
}

#[cfg(test)]
mod tests {
    use super::clamp;

    #[test]
    fn clamp_never_panics() {
        assert_eq!(clamp(5.0, 1.0, 3.0), 3.0);
        assert_eq!(clamp(-5.0, 1.0, 3.0), 1.0);
        assert_eq!(clamp(2.0, 3.0, 1.0), 3.0, "inverted range collapses to lo");
        assert_eq!(clamp(f32::NAN, 1.0, 3.0), 1.0);
        assert_eq!(clamp(2.0, f32::NAN, 3.0), 2.0, "NaN lo ignored");
        assert_eq!(clamp(9.0, 1.0, f32::NAN), 1.0, "NaN hi collapses to lo");
    }
}
