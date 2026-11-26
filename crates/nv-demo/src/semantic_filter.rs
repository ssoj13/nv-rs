//! Viewport category filter: which semantic classes and P42 OSM provenance classes are rendered
//! (the toolbar "eye" dropdown).
//!
//! - **What**:
//!   1. A `u64` mask of hidden semantic ids (`DemoSettings::hidden_semantics`, persisted) and the
//!      one table that groups csg-core's 22 semantic ids for the UI ([`GROUPS`]: Humans, Animals,
//!      Vehicles, Buildings, Roads, Street furniture, Industrial, Water, Ground, Unknown,
//!      Vegetation, and Terrain features).
//!   2. A `u32` mask of hidden P42 OSM provenance classes (`DemoSettings::hidden_osm_classes`,
//!      persisted) over the [`OSM_CLASS_COUNT`] variants of
//!      [`csg_scene::provenance::OsmFeatureClass`] (`Building`, `Road`, `Water`, ...,
//!      `DynamicVehicle`, `DynamicActor`, `DynamicRig`).
//! - **Why**: hiding a class must be exact and complete (gone from camera rays, shadows and
//!   reflections, per part, so Buildings > Windows works) and must not cost the frame when
//!   nothing is hidden; a host-side instance filter could do neither for multi-semantic compounds,
//!   while P42 per-instance OSM provenance maps 1-to-1 to whole instances so OSM class hiding
//!   runs at zero intersection-program cost via hardware OptiX visibility masks.
//! - **How**: the semantic mask reaches the device as `LaunchParams::hidden_semantics`. Every CSG
//!   intersection program resolves the hit's class with csg-device's `csg_instance_semantic`
//!   (instance override, else primitive class) and rejects a hidden one, so the ray continues to
//!   the next surface; the RPN route re-traces past a hidden part, the shadow programs take the
//!   radiance path while the mask is non-zero. With the mask 0 every program pays one uniform
//!   branch. Classic stage instances carry the same class rule on the host
//!   (`csg_scene::inst_flags::resolve_semantic`) and are masked out of the IAS.
//! - **Fast path (per instance, per filter change)**: hiding classes used to cost about +67 % RT
//!   because every intersection program rejected per hit, shadows took the radiance intersector
//!   and RPN compounds re-traced, for every instance. Now each instance is first checked against
//!   the P42 OSM provenance mask ([`osm_prov_hidden`], reading `PackedGpuScene::instance_prov`):
//!   any OSM-hidden instance gets `VisibilityMask = LINES_OCCLUDER_MASK` (`0x80`, zero traversal
//!   and zero IS cost) and is never marked partial. Remaining instances get an effective semantic
//!   mask M ([`instance_semantic_mask`], delegating to csg-rs P41 per-compound `semantic_masks`
//!   plus instance overrides) and [`classify`] sorts them against the hidden semantic mask H into
//!   three classes: [`InstanceFilter::Hidden`] (`M & !H == 0`: visibility mask without the render
//!   bits, zero traversal cost), [`InstanceFilter::Unaffected`] (`M & H == 0`: its intersection
//!   program skips the reject test), [`InstanceFilter::Partial`] (per-hit reject as before). The
//!   partial set reaches the device as a bitset over the packed instance slots
//!   ([`fill_partial_bits`], `LaunchParams::csg_sem_partial`); it is the renderer's filter state,
//!   not csg-rs scene data, and is recomputed only when H, the OSM mask, or the packed instances
//!   change.
//! - **Used by**: `ui.rs` (dropdown and eye badge), `renderer.rs` (launch parameter, classic IAS
//!   mask), `csg_scene_host.rs` (per-instance OSM + semantic classification, partial bitset).

use csg_core::ast::semantic;
use csg_scene::inst_flags;
use csg_scene::provenance::{self, OsmFeatureClass};

/// Semantic ids the pinned csg-core defines (`0..semantic::COUNT`); a new id must be added to a
/// group (the coverage test fails otherwise).
#[cfg(test)]
pub const SEMANTIC_COUNT: u16 = semantic::COUNT;

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
        20 => "Vegetation",
        21 => "Terrain feature",
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
    SemanticGroup { label: "Vegetation", ids: &[semantic::VEGETATION.0], pending: None },
    SemanticGroup { label: "Terrain features", ids: &[semantic::TERRAIN_FEATURE.0], pending: None },
];

/// Number of P42 OSM provenance feature classes defined by `csg-scene`.
///
/// Why: sizes the dense `u32` hidden-class bitmask ([`osm_all_mask`]) directly from the single
/// source of truth (`OsmFeatureClass::ALL`) rather than duplicating a magic constant.
pub const OSM_CLASS_COUNT: usize = OsmFeatureClass::ALL.len();
const _: () = assert!(OSM_CLASS_COUNT < 32);

/// Deterministic bitmask for one [`OsmFeatureClass`] in `OsmFeatureClass::ALL` order (`1u32 << 0..20`).
///
/// Why: `OsmFeatureClass` has sparse `u16` wire discriminants (`0..=12`, `100`, `200..=212`), so
/// packing them into a dense 32-bit bitmask via an exhaustive match gives O(1) toggle/query math
/// and a compile-time error if `csg-scene` ever adds a new variant.
pub const fn osm_class_bit(cls: OsmFeatureClass) -> u32 {
    let idx: u32 = match cls {
        OsmFeatureClass::Unknown => 0,
        OsmFeatureClass::Building => 1,
        OsmFeatureClass::Road => 2,
        OsmFeatureClass::Water => 3,
        OsmFeatureClass::LandUse => 4,
        OsmFeatureClass::Vegetation => 5,
        OsmFeatureClass::Barrier => 6,
        OsmFeatureClass::Industrial => 7,
        OsmFeatureClass::Power => 8,
        OsmFeatureClass::Railway => 9,
        OsmFeatureClass::Marine => 10,
        OsmFeatureClass::StreetFurniture => 11,
        OsmFeatureClass::Poi => 12,
        OsmFeatureClass::Terrain => 13,
        OsmFeatureClass::Procedural => 14,
        OsmFeatureClass::Inferred => 15,
        OsmFeatureClass::Override => 16,
        OsmFeatureClass::DynamicVehicle => 17,
        OsmFeatureClass::DynamicActor => 18,
        OsmFeatureClass::DynamicRig => 19,
    };
    1u32 << idx
}

/// Mask covering every defined [`OsmFeatureClass`] bit (`(1 << OSM_CLASS_COUNT) - 1`).
///
/// Why: lets [`osm_hidden_count`] and bulk hide/show operations ignore unused upper bits.
pub const fn osm_all_mask() -> u32 {
    (1u32 << (OSM_CLASS_COUNT as u32)) - 1
}

/// Returns `true` when a packed P42 `instance_prov` word `(class << 16) | kind` belongs to an
/// OSM feature class currently hidden in `hidden_osm`.
///
/// Why: runs on the host while building OptiX IAS visibility masks and the partial bitset so
/// OSM-hidden instances are culled in hardware (`VisibilityMask = 0x80`) at zero shader cost.
/// Short-circuits on `hidden_osm == 0` so the unfiltered hot path pays one branch.
pub const fn osm_prov_hidden(prov_word: u32, hidden_osm: u32) -> bool {
    if hidden_osm == 0 {
        return false;
    }
    match provenance::unpack_class(prov_word) {
        Some(c) => (hidden_osm & osm_class_bit(c)) != 0,
        None => false,
    }
}

/// Number of hidden P42 OSM provenance classes (ignoring unused bits above [`OSM_CLASS_COUNT`]).
///
/// Why: feeds the toolbar eye badge so hidden OSM classes are surfaced alongside hidden semantics.
pub const fn osm_hidden_count(hidden_osm: u32) -> u32 {
    (hidden_osm & osm_all_mask()).count_ones()
}

/// Returns `hidden_osm` with `cls` shown (`visible = true`, clears bit) or hidden (`visible = false`, sets bit).
///
/// Why: single bit-manipulation helper shared by the eye dropdown toggles and tests.
pub const fn toggle_osm_class(hidden_osm: u32, cls: OsmFeatureClass, visible: bool) -> u32 {
    let b = osm_class_bit(cls);
    if visible { hidden_osm & !b } else { hidden_osm | b }
}

/// Total number of hidden categories across both semantic ids and P42 OSM provenance classes.
///
/// Why: the toolbar eye icon displays one combined badge count so the user always sees when any
/// filter is hiding scene geometry.
pub fn combined_hidden_count(hidden_semantics: u64, hidden_osm: u32) -> u32 {
    hidden_count(hidden_semantics) + osm_hidden_count(hidden_osm)
}

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
/// Why: delegates directly to [`inst_flags::instance_semantic_mask`] (csg-rs P41), preferring the
/// instance's semantic override (including GPU crowd bones, whose override + `CROWD_BONE` bits are
/// written directly by `emit_actors_cuda`) and otherwise using `compound_mask` from
/// `WorldScene::semantic_masks`.
pub const fn instance_semantic_mask(flags: u32, compound_mask: u64) -> u64 {
    inst_flags::instance_semantic_mask(flags, compound_mask)
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
/// while nothing is hidden is unaffected, never hidden. The conservative all-bits sentinel
/// `u64::MAX` (produced by `csg_core::semantic::mask_bit` for semantic ids `>= 64` or when a
/// compound id is out of range) is never classified [`InstanceFilter::Hidden`] even when
/// `hidden == u64::MAX`, so ids outside the 64-bit mask fall through to device per-hit checks and
/// are never hidden.
pub const fn classify(m: u64, hidden: u64) -> InstanceFilter {
    if m & hidden == 0 {
        InstanceFilter::Unaffected
    } else if m != u64::MAX && m & !hidden == 0 {
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
/// [`InstanceFilter::Partial`]) for `instances` using per-compound `semantic_masks` and P42
/// per-instance `instance_prov`, and returns the number of partial slots.
///
/// Why a bitset: the intersection programs read one bit per invocation, 32 slots per cached word
/// (6 KiB for 50k instances). Instances hidden by [`osm_prov_hidden`] are already hardware-culled
/// (`VisibilityMask = 0x80`) and therefore never marked partial. `out` is reused; it grows only
/// with the instance count.
pub fn fill_partial_bits(
    instances: &[csg_scene::GpuInstance],
    semantic_masks: &[u64],
    instance_prov: &[u32],
    hidden_semantics: u64,
    hidden_osm: u32,
    out: &mut Vec<u32>,
) -> usize {
    out.clear();
    out.resize(instances.len().div_ceil(32), 0);
    if hidden_semantics == 0 {
        return 0;
    }
    let mut partial = 0;
    for (i, inst) in instances.iter().enumerate() {
        let prov_word = instance_prov.get(i).copied().unwrap_or(0);
        if osm_prov_hidden(prov_word, hidden_osm) {
            continue;
        }
        let cm = semantic_masks
            .get(inst.compound_id as usize)
            .copied()
            .unwrap_or(u64::MAX);
        if classify(instance_semantic_mask(inst.flags, cm), hidden_semantics) == InstanceFilter::Partial
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
        assert_eq!(SEMANTIC_COUNT, 22);
        assert_eq!(semantic_name(semantic::VEGETATION.0), "Vegetation");
        assert_eq!(semantic_name(semantic::TERRAIN_FEATURE.0), "Terrain feature");
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

    fn gpu_inst(compound_id: u32, flags: u32) -> csg_scene::GpuInstance {
        csg_scene::GpuInstance {
            xform: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            compound_id,
            user_id: 0,
            mat_override: u32::MAX,
            flags,
        }
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
        assert_eq!(classify(u64::MAX, 0), InstanceFilter::Unaffected, "sentinel unaffected when nothing hidden");
        assert_eq!(classify(u64::MAX, u64::MAX), InstanceFilter::Partial, "sentinel is never Hidden");
    }

    #[test]
    fn mask_source_override_compound_and_crowd() {
        let bldg_win = bit(semantic::BUILDING.0) | bit(semantic::WINDOW.0);
        let veg = bit(semantic::VEGETATION.0);
        // Single-class compound: Hidden or Unaffected with no override.
        assert_eq!(instance_semantic_mask(inst_flags::VISIBLE, veg), veg);
        assert_eq!(classify(instance_semantic_mask(inst_flags::VISIBLE, veg), veg), InstanceFilter::Hidden);
        assert_eq!(
            classify(instance_semantic_mask(inst_flags::VISIBLE, veg), bit(semantic::VEHICLE.0)),
            InstanceFilter::Unaffected
        );
        // Multi-class compound: Partial when one sub-class is hidden, Hidden when all are.
        assert_eq!(instance_semantic_mask(inst_flags::VISIBLE, bldg_win), bldg_win);
        assert_eq!(
            classify(instance_semantic_mask(inst_flags::VISIBLE, bldg_win), bit(semantic::WINDOW.0)),
            InstanceFilter::Partial
        );
        assert_eq!(
            classify(instance_semantic_mask(inst_flags::VISIBLE, bldg_win), bldg_win),
            InstanceFilter::Hidden
        );
        // Override beats the compound mask.
        let ped = with_sem(semantic::PEDESTRIAN.0);
        assert_eq!(instance_semantic_mask(ped, bldg_win), bit(semantic::PEDESTRIAN.0));
        assert_eq!(
            classify(instance_semantic_mask(ped, bldg_win), bit(semantic::PEDESTRIAN.0)),
            InstanceFilter::Hidden
        );
        assert_eq!(
            classify(instance_semantic_mask(ped, bldg_win), bit(semantic::BUILDING.0)),
            InstanceFilter::Unaffected
        );
        // Crowd bones carry semantic override directly from `emit_actors_cuda` (no u64::MAX shim).
        let dog_bone = with_sem(semantic::ANIMAL.0) | inst_flags::CROWD_BONE;
        assert_eq!(instance_semantic_mask(dog_bone, bit(semantic::PEDESTRIAN.0)), bit(semantic::ANIMAL.0));
        assert_eq!(
            classify(instance_semantic_mask(dog_bone, bit(semantic::PEDESTRIAN.0)), bit(semantic::ANIMAL.0)),
            InstanceFilter::Hidden
        );
        assert_eq!(
            classify(instance_semantic_mask(dog_bone, bit(semantic::PEDESTRIAN.0)), bit(semantic::PEDESTRIAN.0)),
            InstanceFilter::Unaffected
        );
        // Override id >= 64 yields conservative `u64::MAX` from `csg_core::semantic::mask_bit`,
        // which `classify` keeps out of `Hidden` so ids outside the 64-bit mask are never hidden.
        let high = instance_semantic_mask(with_sem(70), veg);
        assert_eq!(high, u64::MAX);
        assert_eq!(classify(high, 0), InstanceFilter::Unaffected);
        assert_eq!(classify(high, all_mask()), InstanceFilter::Partial);
        assert_eq!(classify(high, u64::MAX), InstanceFilter::Partial);
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
        // Compound masks: 0 = single VEHICLE, 1 = multi VEHICLE|WHEEL, 2 = single ROAD.
        let masks = [
            bit(semantic::VEHICLE.0),
            bit(semantic::VEHICLE.0) | bit(semantic::WHEEL.0),
            bit(semantic::ROAD.0),
        ];
        let base = [
            gpu_inst(0, inst_flags::VISIBLE), // 0 single-class -> hidden (not partial)
            gpu_inst(1, inst_flags::VISIBLE), // 1 multi-class -> partial
            gpu_inst(2, inst_flags::VISIBLE), // 2 unaffected (not partial)
        ];
        let mut many: Vec<csg_scene::GpuInstance> = base.iter().copied().cycle().take(40).collect();
        // Slot 38: multi-class compound overridden to ROAD -> unaffected (not partial).
        many[38] = gpu_inst(1, with_sem(semantic::ROAD.0));
        // Slot 39 (word 1, bit 7): out-of-range compound_id -> conservative u64::MAX -> partial.
        many[39] = gpu_inst(99, inst_flags::VISIBLE);
        let prov = vec![0u32; many.len()];
        let mut out = vec![0xDEAD_BEEF; 7];
        let n = fill_partial_bits(&many, &masks, &prov, hidden, 0, &mut out);
        assert_eq!(out.len(), 2);
        let expect: u32 = (0..32).filter(|i| i % 3 == 1).fold(0, |m, i| m | 1 << i);
        assert_eq!(out[0], expect);
        assert_eq!(out[1], (1 << (34 - 32)) | (1 << (37 - 32)) | (1 << 7));
        assert_eq!(n, expect.count_ones() as usize + 3);
        assert_eq!(fill_partial_bits(&[], &masks, &[], hidden, 0, &mut out), 0);
        assert!(out.is_empty());
    }

    #[test]
    fn osm_class_bits_are_distinct_dense_and_match_all_order() {
        assert_eq!(OSM_CLASS_COUNT, 20);
        let mut covered = 0u32;
        for (idx, &cls) in OsmFeatureClass::ALL.iter().enumerate() {
            let b = osm_class_bit(cls);
            assert_eq!(b, 1u32 << idx, "bit index mismatch for {cls:?}");
            assert_eq!(covered & b, 0, "duplicate bit for {cls:?}");
            covered |= b;
        }
        assert_eq!(covered, osm_all_mask());
        assert_eq!(osm_all_mask(), (1u32 << 20) - 1);
        assert_eq!(osm_hidden_count(0), 0);
        assert_eq!(osm_hidden_count(osm_all_mask()), 20);
        assert_eq!(osm_hidden_count(u32::MAX), 20, "bits outside OSM_CLASS_COUNT do not count");
    }

    #[test]
    fn osm_prov_hidden_toggle_and_combined_count() {
        let bldg = provenance::pack_class_kind(OsmFeatureClass::Building, provenance::prov_kind::building::APARTMENTS);
        let road = provenance::pack_class_kind(OsmFeatureClass::Road, provenance::prov_kind::road::PRIMARY);
        let veh = provenance::pack_class_kind(OsmFeatureClass::DynamicVehicle, provenance::prov_kind::vehicle::ROBOTAXI);
        let unrecognized = 9999u32 << 16;

        assert!(!osm_prov_hidden(bldg, 0));
        assert!(!osm_prov_hidden(unrecognized, osm_all_mask()), "unknown raw class code is never hidden");

        let m1 = toggle_osm_class(0, OsmFeatureClass::Building, false);
        assert_eq!(m1, osm_class_bit(OsmFeatureClass::Building));
        assert!(osm_prov_hidden(bldg, m1));
        assert!(!osm_prov_hidden(road, m1));
        assert!(!osm_prov_hidden(veh, m1));
        assert_eq!(osm_hidden_count(m1), 1);

        let m2 = toggle_osm_class(m1, OsmFeatureClass::DynamicVehicle, false);
        assert!(osm_prov_hidden(bldg, m2));
        assert!(osm_prov_hidden(veh, m2));
        assert_eq!(osm_hidden_count(m2), 2);

        let m3 = toggle_osm_class(m2, OsmFeatureClass::Building, true);
        assert!(!osm_prov_hidden(bldg, m3));
        assert!(osm_prov_hidden(veh, m3));
        assert_eq!(osm_hidden_count(m3), 1);

        let sem = bit(semantic::BUILDING.0) | bit(semantic::WINDOW.0) | bit(50);
        assert_eq!(combined_hidden_count(sem, m2 | (1u32 << 30)), 4);
        assert_eq!(combined_hidden_count(0, 0), 0);
    }

    #[test]
    fn fill_partial_bits_excludes_osm_hidden_instances() {
        let bldg_win = bit(semantic::BUILDING.0) | bit(semantic::WINDOW.0);
        let masks = [bldg_win, bit(semantic::WINDOW.0)];
        let insts = [
            gpu_inst(0, inst_flags::VISIBLE), // 0: partial semantically, hidden by OSM Building -> NOT partial
            gpu_inst(0, inst_flags::VISIBLE), // 1: partial semantically, OSM Override -> partial
            gpu_inst(1, inst_flags::VISIBLE), // 2: hidden semantically, OSM Override -> NOT partial
            gpu_inst(0, inst_flags::VISIBLE), // 3: partial semantically, hidden by OSM Road -> NOT partial
            gpu_inst(0, inst_flags::VISIBLE), // 4: partial semantically, OSM DynamicVehicle -> partial
        ];
        let prov = [
            provenance::pack_class_kind(OsmFeatureClass::Building, provenance::prov_kind::building::OFFICE),
            provenance::pack_class_kind(OsmFeatureClass::Override, provenance::prov_kind::override_kind::PLACED_ASSET),
            provenance::pack_class_kind(OsmFeatureClass::Override, provenance::prov_kind::override_kind::PLACED_ASSET),
            provenance::pack_class_kind(OsmFeatureClass::Road, provenance::prov_kind::road::RESIDENTIAL),
            provenance::pack_class_kind(OsmFeatureClass::DynamicVehicle, provenance::prov_kind::vehicle::SEDAN),
        ];
        let hidden_sem = bit(semantic::WINDOW.0);
        let hidden_osm = osm_class_bit(OsmFeatureClass::Building) | osm_class_bit(OsmFeatureClass::Road);

        let mut out = Vec::new();
        let n = fill_partial_bits(&insts, &masks, &prov, hidden_sem, hidden_osm, &mut out);
        assert_eq!(n, 2);
        assert_eq!(out, vec![(1 << 1) | (1 << 4)]);

        // When only OSM classes are hidden (hidden_semantics == 0), zero slots are partial.
        let n_zero = fill_partial_bits(&insts, &masks, &prov, 0, hidden_osm, &mut out);
        assert_eq!(n_zero, 0);
        assert_eq!(out, vec![0]);

        // Shorter provenance slice defaults missing slots to word 0 (OsmFeatureClass::Unknown).
        let n_short = fill_partial_bits(
            &insts[..2],
            &masks,
            &prov[..1],
            hidden_sem,
            osm_class_bit(OsmFeatureClass::Unknown),
            &mut out,
        );
        assert_eq!(n_short, 1, "slot 0 (Building) stays partial; slot 1 (missing -> Unknown) is OSM-hidden");
        assert_eq!(out, vec![1 << 0]);
    }
}
