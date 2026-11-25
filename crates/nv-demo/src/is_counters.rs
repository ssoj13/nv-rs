//! Intersection-program counters: the measurement side of the csg-rs P37 A/B.
//!
//! - **What**: per OptiX hit group, how many times its intersection program ran in one launch
//!   (CSG general RPN, single primitive / pure union, first-hit-union part, road-tile share,
//!   fractal, sphere; radiance and shadow separately) and how many radiance / shadow rays were
//!   traced. [`IsGroup`] names the groups, [`IsCounts`] holds one launch's totals.
//! - **Why**: P37 replaces hand-assembled box chains (kerbs, walls, rails) by sweep tiles; every
//!   migration wave must show what it does to the intersection work, next to the RT time that
//!   `GpuPassProfiler` already measures, on a fixed set of cameras (`--bench-views`).
//! - **How**: `raytracer.cu` (`nv_is_count`) adds one 64-bit atomic per event into
//!   `LaunchParams::is_counters` (`NV_IS_GROUPS` groups x `NV_IS_STRIPES` stripes, stripe =
//!   launch x & 31, to spread contention). The pointer is 0 unless counting is on, so a normal
//!   launch pays one uniform branch per call site and nothing else. The renderer resets the
//!   buffer before a counted launch, copies it to pinned memory behind an event and sums the
//!   stripes on the next frame ([`IsCounts::from_stripes`]). Counted frames run slower and are
//!   never used for timing.
//! - **Used by**: `renderer.rs` (device rig, `FrameReady::is_counts`), `ui.rs` (Flight stats
//!   toggle and table), `main.rs` (`--is-counters`, `--bench-views` CSV).

/// Counter groups, in device word order (`NV_IS_*` in raytracer.cu).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsGroup {
    /// `__intersection__csg`: general RPN compounds (SBT 8).
    CsgRpn = 0,
    /// `__intersection__csg_shadow`.
    CsgRpnShadow = 1,
    /// `__intersection__csg_prim`: single-primitive and pure-union AABBs (SBT 10).
    CsgPrim = 2,
    /// `__intersection__csg_prim_shadow`.
    CsgPrimShadow = 3,
    /// `__intersection__csg_union_part`: first-hit-union parts (SBT 12).
    UnionPart = 4,
    /// `__intersection__csg_union_part_shadow`.
    UnionPartShadow = 5,
    /// Road / terrain tile primitives among the `CsgPrim` + `CsgPrimShadow` invocations (the
    /// RPN route evaluates road tiles inside csg-rs code and is not split out).
    RoadTile = 6,
    /// `__intersection__fractal` + shadow (classic stage).
    Fractal = 7,
    /// `__intersection__sphere` + shadow (classic stage).
    Sphere = 8,
    /// Radiance rays traced (`trace_primary`: camera and bounce rays).
    RaysRadiance = 9,
    /// Shadow rays traced (`trace_shadow`).
    RaysShadow = 10,
}

impl IsGroup {
    /// Every group in device order (word 11 is reserved and always 0).
    pub const ALL: [Self; 11] = [
        Self::CsgRpn,
        Self::CsgRpnShadow,
        Self::CsgPrim,
        Self::CsgPrimShadow,
        Self::UnionPart,
        Self::UnionPartShadow,
        Self::RoadTile,
        Self::Fractal,
        Self::Sphere,
        Self::RaysRadiance,
        Self::RaysShadow,
    ];

    /// CSV column / UI row label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CsgRpn => "is_csg_rpn",
            Self::CsgRpnShadow => "is_csg_rpn_shadow",
            Self::CsgPrim => "is_csg_prim",
            Self::CsgPrimShadow => "is_csg_prim_shadow",
            Self::UnionPart => "is_union_part",
            Self::UnionPartShadow => "is_union_part_shadow",
            Self::RoadTile => "is_road_tile",
            Self::Fractal => "is_fractal",
            Self::Sphere => "is_sphere",
            Self::RaysRadiance => "rays_radiance",
            Self::RaysShadow => "rays_shadow",
        }
    }

    /// `NV_IS_*` name of the group in raytracer.cu (contract test).
    #[cfg(test)]
    const fn cuda_name(self) -> &'static str {
        match self {
            Self::CsgRpn => "NV_IS_CSG_RPN",
            Self::CsgRpnShadow => "NV_IS_CSG_RPN_SHADOW",
            Self::CsgPrim => "NV_IS_CSG_PRIM",
            Self::CsgPrimShadow => "NV_IS_CSG_PRIM_SHADOW",
            Self::UnionPart => "NV_IS_UNION_PART",
            Self::UnionPartShadow => "NV_IS_UNION_PART_SHADOW",
            Self::RoadTile => "NV_IS_ROAD_TILE",
            Self::Fractal => "NV_IS_FRACTAL",
            Self::Sphere => "NV_IS_SPHERE",
            Self::RaysRadiance => "NV_IS_RAYS_RADIANCE",
            Self::RaysShadow => "NV_IS_RAYS_SHADOW",
        }
    }
}

/// Device groups (`NV_IS_GROUPS`, one reserved).
pub const IS_GROUPS: usize = 12;
/// Stripes per group (`NV_IS_STRIPES`).
pub const IS_STRIPES: usize = 32;
/// 64-bit words of the device counter buffer.
pub const IS_WORDS: usize = IS_GROUPS * IS_STRIPES;

/// One counted launch: invocations / rays per group.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct IsCounts {
    /// Totals indexed by `IsGroup as usize`.
    pub per_group: [u64; IS_GROUPS],
}

impl IsCounts {
    /// Sums the device stripes (`words.len() == IS_WORDS`, group-major).
    pub fn from_stripes(words: &[u64; IS_WORDS]) -> Self {
        let mut per_group = [0u64; IS_GROUPS];
        for (g, total) in per_group.iter_mut().enumerate() {
            *total = words[g * IS_STRIPES..(g + 1) * IS_STRIPES].iter().sum();
        }
        Self { per_group }
    }

    /// Total of one group.
    pub const fn get(&self, g: IsGroup) -> u64 {
        self.per_group[g as usize]
    }

    /// All intersection-program invocations (rays and the road-tile subset excluded).
    pub fn total_is(&self) -> u64 {
        IsGroup::ALL
            .iter()
            .filter(|g| !matches!(g, IsGroup::RoadTile | IsGroup::RaysRadiance | IsGroup::RaysShadow))
            .map(|&g| self.get(g))
            .sum()
    }

    /// Adds another launch's counts (bench views average over several counted frames).
    pub fn accumulate(&mut self, o: &Self) {
        for (a, b) in self.per_group.iter_mut().zip(o.per_group) {
            *a = a.saturating_add(b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_words_match_the_cuda_defines() {
        let cu = include_str!("../shaders/raytracer.cu");
        for g in IsGroup::ALL {
            assert!(cu.contains(&format!("#define {} {}u", g.cuda_name(), g as u32)), "{g:?}");
        }
        assert!(cu.contains(&format!("#define NV_IS_GROUPS {IS_GROUPS}u")));
        assert!(cu.contains(&format!("#define NV_IS_STRIPES {IS_STRIPES}u")));
        assert!(IsGroup::ALL.len() < IS_GROUPS, "the last word is reserved");
        let mut labels: Vec<_> = IsGroup::ALL.iter().map(|g| g.label()).collect();
        labels.dedup();
        assert_eq!(labels.len(), IsGroup::ALL.len());
    }

    #[test]
    fn stripes_sum_per_group_and_totals_skip_rays_and_the_road_subset() {
        let mut w = [0u64; IS_WORDS];
        for s in 0..IS_STRIPES {
            w[IsGroup::CsgPrim as usize * IS_STRIPES + s] = 2;
            w[IsGroup::RoadTile as usize * IS_STRIPES + s] = 1;
            w[IsGroup::RaysRadiance as usize * IS_STRIPES + s] = 5;
        }
        w[IsGroup::CsgRpn as usize * IS_STRIPES + 7] = 3;
        let c = IsCounts::from_stripes(&w);
        assert_eq!(c.get(IsGroup::CsgPrim), 64);
        assert_eq!(c.get(IsGroup::RoadTile), 32);
        assert_eq!(c.get(IsGroup::CsgRpn), 3);
        assert_eq!(c.get(IsGroup::RaysRadiance), 160);
        assert_eq!(c.total_is(), 67);
        let mut sum = c;
        sum.accumulate(&c);
        assert_eq!(sum.get(IsGroup::CsgPrim), 128);
    }
}
