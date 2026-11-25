//! GPU picking for nv-demo: the 32-byte pick record written by the primary launch, the
//! request / readback state machine, and the viewport selection it feeds.
//!
//! - **What**: a click asks the render worker to pick the pixel under the cursor. The next main
//!   OptiX launch carries the pixel (`LaunchParams::pick_px`); the raygen thread of that pixel
//!   writes one [`PickRecordPod`] (instance key, part, distance, world hit point, flags, launch
//!   frame index) from the primary ray it traces anyway. The record is copied to pinned memory
//!   behind an event and consumed on the following frame with a non-blocking `cuEventQuery`.
//! - **Why**: the earlier Ctrl+Click path rebuilt the camera ray on the host and marched every
//!   CSG instance of the scene on the CPU (plus a CUDA crowd readback), a second ray tracer that
//!   could disagree with the image. The GPU record is the image itself: the same lens, the same
//!   culled / LOD-selected instances, the same crowd pose, at zero cost to the frame.
//! - **How**: [`PickState`] holds at most one pending request (a newer click supersedes it, as in
//!   the sibling apps warpbro-rs / squarebob-rs) and one request in flight tagged with the frame
//!   index of its launch; [`PickState::consume`] validates the record against that tag. The
//!   result drives [`Selection`] (plain click selects only the hit object, Shift+click toggles it,
//!   a click on empty space clears unless Shift is held) or possession (Ctrl+click, double click,
//!   Pick mode). Selection keys are the primary-hit instance keys (`hit_instance_id` in
//!   raytracer.cu: the CSG `user_id`, else `0xFF000000 | packed index`), the key space the
//!   overlay filter and the axes highlight use.
//! - **Used by**: `renderer.rs` (render loop: request, launch parameters, readback, consume),
//!   `ui.rs` (click gestures), `main.rs` (forwarding, status line).

use bytemuck::{Pod, Zeroable};

/// `LaunchParams::pick_px` value meaning "no pick this launch" (`NV_PICK_NONE` in raytracer.cu).
pub const PICK_NONE: u32 = u32::MAX;

/// Flag bits of [`PickRecordPod::flags`] (`NV_PICK_FLAG_*` in raytracer.cu).
pub mod pick_flags {
    /// The primary ray hit geometry (otherwise sky, or outside a fisheye circle).
    pub const HIT: u32 = 1;
    /// The hit is a CSG instance (otherwise classic demo geometry).
    pub const CSG: u32 = 2;
}

/// The 32-byte record the primary launch writes for the picked pixel; layout = `NvPickRecord`
/// in `shaders/raytracer.cu` (size and offsets asserted on both sides).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct PickRecordPod {
    /// Primary-hit instance key (`hit_instance_id`), valid with [`pick_flags::HIT`].
    pub inst_id: u32,
    /// CSG part id of the hit primitive (`u32::MAX` for classic geometry).
    pub part_id: u32,
    /// Distance from the camera to the hit point, metres.
    pub t: f32,
    /// World-space hit point, metres.
    pub world_pos: [f32; 3],
    /// [`pick_flags`] bits.
    pub flags: u32,
    /// `LaunchParams::frame_index` of the launch that wrote the record; the host checks it
    /// against the launch that carried its request, so a stale record is never consumed.
    pub frame: u32,
}
const _: () = assert!(std::mem::size_of::<PickRecordPod>() == 32);
const _: () = assert!(std::mem::offset_of!(PickRecordPod, part_id) == 4);
const _: () = assert!(std::mem::offset_of!(PickRecordPod, t) == 8);
const _: () = assert!(std::mem::offset_of!(PickRecordPod, world_pos) == 12);
const _: () = assert!(std::mem::offset_of!(PickRecordPod, flags) == 24);
const _: () = assert!(std::mem::offset_of!(PickRecordPod, frame) == 28);

impl PickRecordPod {
    /// The record says the primary ray hit something.
    pub const fn is_hit(&self) -> bool {
        self.flags & pick_flags::HIT != 0
    }

    /// The hit is a CSG scene instance (only those can be possessed).
    pub const fn is_csg(&self) -> bool {
        self.flags & pick_flags::CSG != 0
    }
}

/// What a viewport click asks for (sibling-app interaction model: warpbro-rs `sl-studio`
/// `click_select`, squarebob-rs `treemap_view`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PickAction {
    /// Plain click (`add = false`): select only the hit object, clear on a miss. Shift+click
    /// (`add = true`): toggle the hit object, keep the selection on a miss.
    Select { add: bool },
    /// Ctrl+click, double click or Pick mode: possess the hit object's camera mount.
    Possess,
}

/// One click to resolve: the normalised viewport position and what to do with the hit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PickRequest {
    /// Horizontal position in `[0, 1]` of the rendered image.
    pub norm_x: f32,
    /// Vertical position in `[0, 1]` (0 = top).
    pub norm_y: f32,
    /// What the hit is used for.
    pub action: PickAction,
}

impl PickRequest {
    /// Pixel of a `w x h` launch under the click, packed as `LaunchParams::pick_px`
    /// (`x | y << 16`). Resolutions are capped at 3840 x 2160, far below 16 bits.
    pub fn packed_pixel(&self, w: u32, h: u32) -> u32 {
        let px = |n: f32, size: u32| -> u32 {
            let size = size.max(1);
            ((n.clamp(0.0, 1.0) * size as f32) as u32).min(size - 1)
        };
        px(self.norm_x, w) | (px(self.norm_y, h) << 16)
    }
}

/// Outcome of [`PickState::consume`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PickOutcome {
    /// The record of the request's launch: act on it.
    Ready(PickRequest, PickRecordPod),
    /// The launch did not write the record (its frame tag differs): the click is dropped and
    /// reported, never answered with another frame's record.
    Lost(PickRequest),
}

/// Request / readback state of the GPU pick (render thread only).
#[derive(Debug, Default)]
pub struct PickState {
    pending: Option<PickRequest>,
    in_flight: Option<(PickRequest, u32)>,
}

impl PickState {
    /// Queues a click; a newer click replaces one that has not been launched yet.
    pub fn request(&mut self, req: PickRequest) {
        self.pending = Some(req);
    }

    /// Nothing queued and nothing in flight: the renderer may idle (converged accumulation)
    /// without starving a click.
    pub const fn is_idle(&self) -> bool {
        self.pending.is_none() && self.in_flight.is_none()
    }

    /// A record is being read back.
    pub const fn in_flight(&self) -> bool {
        self.in_flight.is_some()
    }

    /// Takes the pending request for the launch tagged `frame` and returns its packed pixel,
    /// or [`PICK_NONE`] when there is nothing to pick or a record is still in flight (one pick
    /// per readback keeps the single 32-byte record slot race-free).
    pub fn launch(&mut self, frame: u32, w: u32, h: u32) -> u32 {
        if self.in_flight.is_some() {
            return PICK_NONE;
        }
        match self.pending.take() {
            Some(req) => {
                self.in_flight = Some((req, frame));
                req.packed_pixel(w, h)
            }
            None => PICK_NONE,
        }
    }

    /// Hands the read-back `record` to the request in flight. `None` when nothing is in flight.
    pub fn consume(&mut self, record: PickRecordPod) -> Option<PickOutcome> {
        let (req, frame) = self.in_flight.take()?;
        Some(if record.frame == frame {
            PickOutcome::Ready(req, record)
        } else {
            PickOutcome::Lost(req)
        })
    }

    /// Drops everything (scene switch, rig rebuilt): an old click must not land in a new scene.
    pub fn reset(&mut self) {
        self.pending = None;
        self.in_flight = None;
    }
}

/// OSM provenance of a picked object (feature class, kind, OSM id). Typed slot for csg-rs P42
/// (per-instance provenance record with a device copy); csg-scene carries no provenance yet, so
/// every pick reports `None` and the UI says so instead of guessing from names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OsmProvenance {
    /// `OsmFeatureClass` discriminant (osm-rs feature family: road, building, water, ...).
    pub class: u16,
    /// Kind within the class (osm-rs `RoadKind`, `BuildingKind`, ...).
    pub kind: u16,
    /// OSM element id.
    pub osm_id: u64,
}

/// The scene object behind a pick, as reported to the UI.
#[derive(Clone, Debug, PartialEq)]
pub struct PickedObject {
    /// Selection key (primary-hit instance key).
    pub key: u32,
    /// Assembly / actor display name, or `Object #key` when the scene has no instance for it.
    pub name: String,
    /// CSG part id of the hit primitive.
    pub part_id: u32,
    /// World hit point.
    pub world_pos: [f32; 3],
    /// OSM provenance; `None` until csg-scene publishes it (P42).
    pub osm: Option<OsmProvenance>,
}

/// The viewport selection: instance keys in click order; the last one is the active object.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Selection {
    keys: Vec<u32>,
}

impl Selection {
    /// Selection keys, the last being the active object.
    pub fn keys(&self) -> &[u32] {
        &self.keys
    }

    /// The most recently selected object.
    pub fn active(&self) -> Option<u32> {
        self.keys.last().copied()
    }

    /// Applies a select click: `hit` is the picked key (`None` = empty space). Returns whether
    /// the selection changed. Plain click: select only `hit` (clear on a miss). `add`
    /// (Shift): toggle `hit`, keep the rest; a miss keeps everything.
    pub fn click(&mut self, hit: Option<u32>, add: bool) -> bool {
        let before = self.keys.len();
        match (hit, add) {
            (None, true) => false,
            (None, false) => {
                self.keys.clear();
                before != 0
            }
            (Some(k), true) => {
                if let Some(i) = self.keys.iter().position(|&x| x == k) {
                    self.keys.remove(i);
                } else {
                    self.keys.push(k);
                }
                true
            }
            (Some(k), false) => {
                let same = self.keys.len() == 1 && self.keys[0] == k;
                if !same {
                    self.keys.clear();
                    self.keys.push(k);
                }
                !same
            }
        }
    }

    /// Replaces the selection with `key` (possession selects the possessed object).
    pub fn select_only(&mut self, key: u32) -> bool {
        self.click(Some(key), false)
    }

    /// Clears the selection; returns whether it held anything.
    pub fn clear(&mut self) -> bool {
        self.click(None, false)
    }
}

/// Current selection as reported to the UI (`RenderEvent::SelectionChanged`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectionInfo {
    /// Selected keys, the last being the active object.
    pub keys: Vec<u32>,
    /// The object the last pick resolved to (`None` after a clear or a miss).
    pub active: Option<PickedObject>,
}

/// Status-line text for a selection change.
pub fn selection_status(sel: &SelectionInfo) -> String {
    match (&sel.active, sel.keys.len()) {
        (_, 0) => "Selection cleared".to_string(),
        (Some(a), 1) => format!(
            "Selected {} (key {:#010x}, part {}) at ({:.1}, {:.1}, {:.1})",
            a.name, a.key, a.part_id, a.world_pos[0], a.world_pos[1], a.world_pos[2]
        ),
        (Some(a), n) => format!("{n} objects selected; last: {} (key {:#010x})", a.name, a.key),
        (None, n) => format!("{n} objects selected"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(frame: u32, key: u32) -> PickRecordPod {
        PickRecordPod {
            inst_id: key,
            part_id: 3,
            t: 12.5,
            world_pos: [1.0, 2.0, 3.0],
            flags: pick_flags::HIT | pick_flags::CSG,
            frame,
        }
    }

    const SEL: PickRequest = PickRequest { norm_x: 0.5, norm_y: 0.25, action: PickAction::Select { add: false } };

    #[test]
    fn record_layout_matches_the_cuda_struct() {
        let cu = include_str!("../shaders/raytracer.cu");
        assert!(cu.contains("static_assert(sizeof(NvPickRecord) == 32"));
        for (field, off) in [("part_id", 4), ("t", 8), ("world_pos", 12), ("flags", 24), ("frame", 28)] {
            assert!(
                cu.contains(&format!("__builtin_offsetof(NvPickRecord, {field}) == {off}")),
                "NvPickRecord::{field} offset {off}"
            );
        }
        for (name, v) in [
            ("NV_PICK_FLAG_HIT", pick_flags::HIT),
            ("NV_PICK_FLAG_CSG", pick_flags::CSG),
            ("NV_PICK_NONE", PICK_NONE),
        ] {
            assert!(cu.contains(&format!("#define {name} {v:#X}u")) || cu.contains(&format!("#define {name} {v}u")), "{name}");
        }
    }

    #[test]
    fn pixel_packing_clamps_to_the_launch() {
        assert_eq!(SEL.packed_pixel(200, 100), 100 | (25 << 16));
        let edge = PickRequest { norm_x: 1.0, norm_y: 1.0, ..SEL };
        assert_eq!(edge.packed_pixel(200, 100), 199 | (99 << 16));
        let neg = PickRequest { norm_x: -3.0, norm_y: f32::NAN, ..SEL };
        assert_eq!(neg.packed_pixel(200, 100), 0, "NaN / negative clamp to the first pixel");
    }

    #[test]
    fn one_pick_in_flight_newest_click_wins_and_stale_records_are_rejected() {
        let mut s = PickState::default();
        assert!(s.is_idle());
        assert_eq!(s.launch(7, 200, 100), PICK_NONE, "nothing queued");
        s.request(SEL);
        let newer = PickRequest { norm_x: 0.0, ..SEL };
        s.request(newer);
        assert_eq!(s.launch(7, 200, 100), 25 << 16, "the newer click replaced the older one");
        assert!(s.in_flight() && !s.is_idle());
        s.request(SEL);
        assert_eq!(s.launch(8, 200, 100), PICK_NONE, "no second pick while one is read back");
        assert_eq!(s.consume(rec(7, 42)), Some(PickOutcome::Ready(newer, rec(7, 42))));
        assert_eq!(s.launch(9, 200, 100), 100 | (25 << 16), "the queued click goes next");
        assert_eq!(s.consume(rec(3, 42)), Some(PickOutcome::Lost(SEL)), "another frame's record");
        assert_eq!(s.consume(rec(9, 42)), None, "nothing in flight");
        s.request(SEL);
        s.reset();
        assert!(s.is_idle());
    }

    #[test]
    fn selection_follows_the_click_model() {
        let mut sel = Selection::default();
        assert!(!sel.click(None, false), "clearing an empty selection changes nothing");
        assert!(sel.click(Some(1), false));
        assert!(!sel.click(Some(1), false), "re-selecting the only object changes nothing");
        assert!(sel.click(Some(2), true));
        assert!(sel.click(Some(3), true));
        assert_eq!(sel.keys(), &[1, 2, 3]);
        assert_eq!(sel.active(), Some(3));
        assert!(sel.click(Some(2), true), "Shift toggles a member out");
        assert_eq!(sel.keys(), &[1, 3]);
        assert!(!sel.click(None, true), "Shift + empty space keeps the selection");
        assert!(sel.click(Some(9), false), "plain click selects only the hit");
        assert_eq!(sel.keys(), &[9]);
        assert!(sel.click(None, false), "plain click on empty space clears");
        assert!(sel.keys().is_empty());
        assert!(sel.select_only(5) && sel.clear() && sel.active().is_none());
    }

    #[test]
    fn status_names_the_active_object() {
        let obj = PickedObject { key: 7, name: "robotaxi".into(), part_id: 2, world_pos: [1.0, 2.0, 3.0], osm: None };
        let one = SelectionInfo { keys: vec![7], active: Some(obj.clone()) };
        assert!(selection_status(&one).starts_with("Selected robotaxi (key 0x00000007, part 2)"));
        let many = SelectionInfo { keys: vec![1, 7], active: Some(obj) };
        assert!(selection_status(&many).starts_with("2 objects selected; last: robotaxi"));
        assert_eq!(selection_status(&SelectionInfo::default()), "Selection cleared");
    }

    #[test]
    fn record_flags_decode() {
        let r = rec(1, 1);
        assert!(r.is_hit() && r.is_csg());
        let miss = PickRecordPod { flags: 0, ..r };
        assert!(!miss.is_hit() && !miss.is_csg());
    }
}
