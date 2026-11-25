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
//! - **Used by**: `ui.rs` (dropdown), `renderer.rs` (launch parameter, classic IAS mask).

use csg_core::ast::semantic;

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
        use csg_scene::inst_flags;
        let flags = inst_flags::with_semantic(0, Some(semantic::ANIMAL)).expect("fits");
        assert_eq!(inst_flags::resolve_semantic(flags, semantic::PEDESTRIAN), semantic::ANIMAL);
        assert_eq!(inst_flags::resolve_semantic(0, semantic::PEDESTRIAN), semantic::PEDESTRIAN);
        let mask = set_group(0, &GROUPS[1], false);
        assert_ne!(mask & bit(inst_flags::resolve_semantic(flags, semantic::PEDESTRIAN).0), 0);
        let cu = include_str!("../shaders/raytracer.cu");
        assert!(cu.contains("csg_instance_semantic(flags, hit.sem_id)"));
        assert!(cu.contains("static_assert(sizeof(Params) == 672"));
    }
}
