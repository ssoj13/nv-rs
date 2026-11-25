//! Viewport category filter: which semantic classes are rendered (the toolbar "eye" dropdown).
//!
//! - **What**: a `u64` mask of hidden semantic ids (`DemoSettings::hidden_semantics`, persisted)
//!   and the one table that groups csg-core's semantic ids for the UI ([`GROUPS`]: Humans,
//!   Animals, Vehicles, Buildings, Roads, Street furniture, Industrial, Water, Ground, Unknown,
//!   plus Vegetation / Terrain features that wait for csg-rs P41). The OSM class section of the
//!   dropdown waits for per-instance provenance (csg-rs P42) and says so ([`OSM_SECTION_STATUS`]).
//! - **Why**: hiding a class must be exact and complete (gone from camera rays, shadows and
//!   reflections, per part, so Buildings > Windows works) and must not cost the frame when
//!   nothing is hidden; a host-side instance filter could do neither.
//! - **How**: the mask reaches the device as `LaunchParams::hidden_semantics`. Every CSG
//!   intersection program resolves the hit's class with csg-device's `csg_instance_semantic`
//!   (instance override, else primitive class) and rejects a hidden one, so the ray continues to
//!   the next surface; the RPN route re-traces past a hidden part, the shadow programs take the
//!   radiance path while the mask is non-zero. With the mask 0 every program pays one uniform
//!   branch. Classic stage instances carry the same class rule on the host
//!   (`csg_scene::inst_flags::resolve_semantic`) and are masked out of the IAS.
//! - **Fast path (per instance, per filter change)**: hiding classes used to cost about +67 % RT
//!   because every intersection program rejected per hit, shadows took the radiance intersector
//!   and RPN compounds re-traced, for every instance. Now each instance gets an effective
//!   semantic mask M ([`instance_semantic_mask`], a conservative superset of the classes its hits
//!   can carry) and [`classify`] sorts it against the hidden mask H into three classes:
//!   [`InstanceFilter::Hidden`] (`M & !H == 0`: visibility mask without the render bits, zero
//!   traversal cost), [`InstanceFilter::Unaffected`] (`M & H == 0`: its intersection program
//!   skips the reject test), [`InstanceFilter::Partial`] (per-hit reject as before). The partial
//!   set reaches the device as a bitset over the packed instance slots ([`fill_partial_bits`],
//!   `LaunchParams::csg_sem_partial`); it is the renderer's filter state, not csg-rs scene data,
//!   and is recomputed only when H or the packed instances change.
//! - **Used by**: `ui.rs` (dropdown), `renderer.rs` (launch parameter, classic IAS mask),
//!   `csg_scene_host.rs` (per-instance classification, partial bitset).

use csg_core::ast::semantic;
use csg_scene::inst_flags;

/// Semantic ids the pinned csg-core defines (`0..=ANIMAL`); a new id must be added to a group
/// (the coverage test fails otherwise).
#[cfg(test)]
pub const SEMANTIC_COUNT: u16 = semantic::ANIMAL.0 + 1;

/// Display name of a semantic id (sub-rows of the dropdown).
pub const fn semantic_name(id: u16) -> &'static str {
    match id {
        0 => "Unknown",
        1 => "Road",
        2 => "Sidewalk",
        3 => "Curb",
        4 => "Building",
        5 => "Window",
        6 => "Pole",
        7 => "Traffic light",
        8 => "Traffic sign",
        9 => "Furniture",
        10 => "Vehicle",
        11 => "Wheel",
        12 => "Industrial",
        13 => "Pipe",
        14 => "Tank",
        15 => "Water",
        16 => "Ground",
        17 => "Barrier",
        18 => "Pedestrian",
        19 => "Animal",
        _ => "?",
    }
}

/// One row of the dropdown: a label and the semantic ids it hides, or the reason it has none.
#[derive(Clone, Copy, Debug)]
pub struct SemanticGroup {
    /// Row label.
    pub label: &'static str,
    /// Semantic ids of the group (sub-rows when more than one).
    pub ids: &'static [u16],
    /// Why the group has no ids yet (shown instead of a toggle), `None` when active.
    pub pending: Option<&'static str>,
}

/// The group table, the single place that maps csg-core semantics to UI categories.
pub const GROUPS: &[SemanticGroup] = &[
    SemanticGroup { label: "Humans", ids: &[semantic::PEDESTRIAN.0], pending: None },
    SemanticGroup { label: "Animals", ids: &[semantic::ANIMAL.0], pending: None },
    SemanticGroup { label: "Vehicles", ids: &[semantic::VEHICLE.0, semantic::WHEEL.0], pending: None },
    SemanticGroup { label: "Buildings", ids: &[semantic::BUILDING.0, semantic::WINDOW.0], pending: None },
    SemanticGroup {
        label: "Roads",
        ids: &[semantic::ROAD.0, semantic::SIDEWALK.0, semantic::CURB.0],
        pending: None,
    },
    SemanticGroup {
        label: "Street furniture",
        ids: &[
            semantic::POLE.0,
            semantic::TRAFFIC_LIGHT.0,
            semantic::TRAFFIC_SIGN.0,
            semantic::FURNITURE.0,
            semantic::BARRIER.0,
        ],
        pending: None,
    },
    SemanticGroup {
        label: "Industrial",
        ids: &[semantic::INDUSTRIAL.0, semantic::PIPE.0, semantic::TANK.0],
        pending: None,
    },
    SemanticGroup { label: "Water", ids: &[semantic::WATER.0], pending: None },
    SemanticGroup { label: "Ground", ids: &[semantic::GROUND.0], pending: None },
    SemanticGroup { label: "Unknown", ids: &[semantic::UNKNOWN.0], pending: None },
    // csg-rs P41 adds VEGETATION = 20 and TERRAIN_FEATURE = 21 (trees and scrub are tagged
    // FURNITURE until then); the rows activate when the pinned csg-core defines the ids.
    SemanticGroup { label: "Vegetation", ids: &[], pending: Some("needs csg-rs P41") },
    SemanticGroup { label: "Terrain features", ids: &[], pending: Some("needs csg-rs P41") },
];

/// Status of the OSM class section: csg-scene instances carry no OSM class / kind / id yet.
pub const OSM_SECTION_STATUS: &str = "needs csg-scene OSM provenance (csg-rs P42)";

/// Mask bit of a semantic id (csg-rs ABI invariant: ids are below 64).
pub const fn bit(id: u16) -> u64 {
    1u64 << (id as u32 & 63)
}

/// Mask of all ids of `g`.
pub fn group_mask(g: &SemanticGroup) -> u64 {
    g.ids.iter().fold(0, |m, &id| m | bit(id))
}

/// Mask of every id the table covers.
pub fn all_mask() -> u64 {
    GROUPS.iter().fold(0, |m, g| m | group_mask(g))
}

/// How much of a group a mask hides.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hidden {
    /// Nothing of the group is hidden.
    None,
    /// Some ids are hidden.
    Some,
    /// The whole group is hidden.
    All,
}

/// Hidden state of `g` under `mask`.
pub fn group_hidden(mask: u64, g: &SemanticGroup) -> Hidden {
    let gm = group_mask(g);
    match mask & gm {
        0 => Hidden::None,
        x if x == gm => Hidden::All,
        _ => Hidden::Some,
    }
}

/// `mask` with the whole group shown (`visible`) or hidden.
pub fn set_group(mask: u64, g: &SemanticGroup, visible: bool) -> u64 {
    if visible { mask & !group_mask(g) } else { mask | group_mask(g) }
}

/// Number of hidden semantic ids (the badge on the eye icon).
pub fn hidden_count(mask: u64) -> u32 {
    (mask & all_mask()).count_ones()
}

/// OptiX instance visibility bit reserved for csg-rs `LINES_OCCLUDER` rays (OVERLAY:NEXT); render
/// rays trace with the other seven bits (`NV_RENDER_RAY_MASK` in raytracer.cu), so a hidden
/// instance keeps this bit and stays an occluder for the line pass.
pub const LINES_OCCLUDER_MASK: u8 = 0x80;

/// Effective semantic mask M of an instance: bit `s` set when a hit on it may carry semantic `s`
/// (a superset is correct, only slower). The single source of M.
///
/// Why this shape: the pinned csg-rs (`0fb1087`) has no per-compound semantic mask yet, so only
/// an instance's semantic override (flags bits 16..24 = id + 1) gives an exact answer; every
/// other instance is unknown (all bits: always [`InstanceFilter::Partial`], the per-hit reject of
/// before). Crowd bones are unknown too: the GPU crowd scatter (`cuda_crowd.rs`) rewrites their
/// device records every frame without the host override, so their device class is the
/// primitive's. csg-rs P41 replaces the body with
/// `csg_scene::instance_semantic_mask(flags, library.semantic_masks[compound_id])`.
pub const fn instance_semantic_mask(flags: u32) -> u64 {
    if flags & inst_flags::CROWD_BONE != 0 {
        return u64::MAX;
    }
    match inst_flags::semantic(flags) {
        // Ids >= 64 are outside the hidden mask (csg-rs ABI): such a class is never hidden.
        Some(s) if s.0 < 64 => bit(s.0),
        Some(_) => 0,
        None => u64::MAX,
    }
}

/// What the category filter does with one instance this frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstanceFilter {
    /// No class of the instance is hidden: the intersection program skips the reject test.
    Unaffected,
    /// Some, not all, of its classes are hidden: per-hit reject on the device.
    Partial,
    /// Every class it can carry is hidden: removed from render rays by its visibility mask.
    Hidden,
}

/// Three-way classification of an instance with effective mask `m` under hidden mask `hidden`.
///
/// Unaffected is tested first, so an instance with no classes (`m == 0`) and every instance
/// while nothing is hidden is unaffected, never hidden.
pub const fn classify(m: u64, hidden: u64) -> InstanceFilter {
    if m & hidden == 0 {
        InstanceFilter::Unaffected
    } else if m & !hidden == 0 {
        InstanceFilter::Hidden
    } else {
        InstanceFilter::Partial
    }
}

/// OptiX visibility mask of an instance whose unfiltered mask is `base`: a hidden instance keeps
/// only the [`LINES_OCCLUDER_MASK`] bit, so no render ray traverses it.
pub const fn visibility_mask(base: u8, f: InstanceFilter) -> u8 {
    match f {
        InstanceFilter::Hidden => base & LINES_OCCLUDER_MASK,
        InstanceFilter::Unaffected | InstanceFilter::Partial => base,
    }
}

/// Fills `out` with the device partial bitset (bit `i` of word `i / 32` = instance slot `i` is
/// [`InstanceFilter::Partial`]) for the instances whose flags `flags` yields in slot order, and
/// returns the number of partial slots.
///
/// Why a bitset: the intersection programs read one bit per invocation, 32 slots per cached word
/// (6 KiB for 50k instances). `out` is reused; it grows only with the instance count.
pub fn fill_partial_bits(flags: impl ExactSizeIterator<Item = u32>, hidden: u64, out: &mut Vec<u32>) -> usize {
    out.clear();
    out.resize(flags.len().div_ceil(32), 0);
    let mut partial = 0;
    for (i, f) in flags.enumerate() {
        if classify(instance_semantic_mask(f), hidden) == InstanceFilter::Partial
            && let Some(w) = out.get_mut(i / 32)
        {
            *w |= 1 << (i % 32);
            partial += 1;
        }
    }
    partial
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_cover_every_semantic_exactly_once() {
        let mut seen = [0u32; 64];
        for g in GROUPS {
            assert_eq!(g.ids.is_empty(), g.pending.is_some(), "{}: ids xor pending reason", g.label);
            for &id in g.ids {
                assert!(id < 64, "csg-rs ABI: semantic ids are below 64 ({id})");
                seen[id as usize] += 1;
            }
        }
        for id in 0..SEMANTIC_COUNT {
            assert_eq!(seen[id as usize], 1, "semantic {id} ({}) must be in exactly one group", semantic_name(id));
            assert_ne!(semantic_name(id), "?");
        }
        assert!(seen[SEMANTIC_COUNT as usize..].iter().all(|&n| n == 0), "no ids beyond csg-core's");
        assert_eq!(all_mask(), (1u64 << SEMANTIC_COUNT) - 1);
    }

    #[test]
    fn mask_math() {
        let roads = &GROUPS[4];
        assert_eq!(roads.label, "Roads");
        let m = set_group(0, roads, false);
        assert_eq!(m, bit(1) | bit(2) | bit(3));
        assert_eq!(group_hidden(m, roads), Hidden::All);
        assert_eq!(group_hidden(m & !bit(2), roads), Hidden::Some);
        assert_eq!(group_hidden(0, roads), Hidden::None);
        assert_eq!(set_group(m, roads, true), 0);
        assert_eq!(hidden_count(m | bit(40)), 3, "bits outside the table do not count");
        let buildings = &GROUPS[3];
        let windows_only = bit(semantic::WINDOW.0);
        assert_eq!(group_hidden(windows_only, buildings), Hidden::Some, "per-id hiding (Buildings > Windows)");
    }

    #[test]
    fn device_reject_rule_matches_the_csg_semantic_resolution() {
        // The kernels test `(mask >> csg_instance_semantic(flags, prim_sem)) & 1`; the host twin
        // of csg_instance_semantic is `inst_flags::resolve_semantic`.
        let flags = inst_flags::with_semantic(0, Some(semantic::ANIMAL)).expect("fits");
        assert_eq!(inst_flags::resolve_semantic(flags, semantic::PEDESTRIAN), semantic::ANIMAL);
        assert_eq!(inst_flags::resolve_semantic(0, semantic::PEDESTRIAN), semantic::PEDESTRIAN);
        let mask = set_group(0, &GROUPS[1], false);
        assert_ne!(mask & bit(inst_flags::resolve_semantic(flags, semantic::PEDESTRIAN).0), 0);
        let cu = include_str!("../shaders/raytracer.cu");
        assert!(cu.contains("csg_instance_semantic(flags, hit.sem_id)"));
        assert!(cu.contains("static_assert(sizeof(Params) == 680"));
    }

    fn with_sem(id: u16) -> u32 {
        inst_flags::with_semantic(inst_flags::VISIBLE, Some(csg_core::SemanticId(id))).expect("fits")
    }

    #[test]
    fn classify_all_none_partial() {
        let m = bit(semantic::VEHICLE.0) | bit(semantic::WHEEL.0);
        assert_eq!(classify(m, 0), InstanceFilter::Unaffected, "nothing hidden");
        assert_eq!(classify(m, bit(semantic::ROAD.0)), InstanceFilter::Unaffected, "other classes hidden");
        assert_eq!(classify(m, m), InstanceFilter::Hidden, "all of its classes hidden");
        assert_eq!(classify(m, m | bit(semantic::ROAD.0)), InstanceFilter::Hidden, "superset hidden");
        assert_eq!(classify(m, bit(semantic::WHEEL.0)), InstanceFilter::Partial, "some hidden");
        assert_eq!(classify(0, u64::MAX), InstanceFilter::Unaffected, "no classes is never hidden");
    }

    #[test]
    fn mask_source_override_unknown_crowd() {
        // Override: exactly its class.
        let ped = with_sem(semantic::PEDESTRIAN.0);
        assert_eq!(instance_semantic_mask(ped), bit(semantic::PEDESTRIAN.0));
        assert_eq!(classify(instance_semantic_mask(ped), bit(semantic::PEDESTRIAN.0)), InstanceFilter::Hidden);
        assert_eq!(classify(instance_semantic_mask(ped), bit(semantic::VEHICLE.0)), InstanceFilter::Unaffected);
        // Unknown (no override, csg-rs P41 pending): always partial while anything is hidden.
        assert_eq!(instance_semantic_mask(inst_flags::VISIBLE), u64::MAX);
        assert_eq!(classify(u64::MAX, bit(semantic::VEHICLE.0)), InstanceFilter::Partial);
        assert_eq!(classify(u64::MAX, 0), InstanceFilter::Unaffected);
        // Crowd bones: the GPU scatter drops the override, so they stay unknown.
        assert_eq!(instance_semantic_mask(ped | inst_flags::CROWD_BONE), u64::MAX);
        // An override id outside the 64-bit mask can never be hidden.
        assert_eq!(classify(instance_semantic_mask(with_sem(70)), u64::MAX), InstanceFilter::Unaffected);
    }

    #[test]
    fn visibility_keeps_only_the_lines_occluder_bit() {
        assert_eq!(visibility_mask(0xFF, InstanceFilter::Hidden), LINES_OCCLUDER_MASK);
        assert_eq!(visibility_mask(0x7F, InstanceFilter::Hidden), 0);
        assert_eq!(visibility_mask(0xFF, InstanceFilter::Partial), 0xFF);
        assert_eq!(visibility_mask(0xFF, InstanceFilter::Unaffected), 0xFF);
        let cu = include_str!("../shaders/raytracer.cu");
        assert!(cu.contains("#define NV_RENDER_RAY_MASK 0x7Fu"), "render rays must skip the LINES_OCCLUDER bit");
        assert!(!cu.contains("OptixVisibilityMask(255)"), "every trace uses NV_RENDER_RAY_MASK");
    }

    #[test]
    fn partial_bitset_marks_only_partial_slots() {
        let hidden = bit(semantic::VEHICLE.0);
        let flags = [
            with_sem(semantic::VEHICLE.0),    // 0 hidden
            inst_flags::VISIBLE,              // 1 unknown -> partial
            with_sem(semantic::ROAD.0),       // 2 unaffected
        ];
        let mut many: Vec<u32> = flags.iter().copied().cycle().take(40).collect();
        many[39] = inst_flags::VISIBLE; // slot 39 (word 1, bit 7) partial
        let mut out = vec![0xDEAD_BEEF; 7];
        let n = fill_partial_bits(many.iter().copied(), hidden, &mut out);
        assert_eq!(out.len(), 2);
        let expect: u32 = (0..32).filter(|i| i % 3 == 1).fold(0, |m, i| m | 1 << i);
        assert_eq!(out[0], expect);
        assert_eq!(out[1], (1 << (34 - 32)) | (1 << (37 - 32)) | (1 << 7));
        assert_eq!(n, expect.count_ones() as usize + 3);
        assert_eq!(fill_partial_bits(std::iter::empty(), hidden, &mut out), 0);
        assert!(out.is_empty());
    }
}
