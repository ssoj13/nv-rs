//! Rail fly-through statistics shown in the "Cinema & Camera Rail" panel (Flight stats).
//!
//! **What.** [`FlightStats`] accumulates one entry per rendered frame while the camera rail
//! drives the camera: the frame's [`PerfSample`] (the same struct the render worker hands to the
//! perf CSV, so there is one source for every timing and launch-state value), the rail time and
//! the scene load of the frame (visible CSG instances, LOD counts, evaluation paths). It derives
//! live camera speed / angular speed, accumulated RT min / median / p95 / max, an RT histogram,
//! the five slowest rail positions, and the per-loop curves (current loop and previous loop) for
//! the RT-vs-rail-time and scene-load graphs; it exports the accumulated frames as the perf CSV.
//! **Why.** A rail loop renders a repeatable camera sequence; seeing where on the rail the RT
//! pass is slow, whether it is stable loop to loop, and what scene load comes with it, is the
//! interactive counterpart of `--bench-rail`.
//! **How.** All buffers are allocated once in [`FlightStats::new`] (ring buffers of fixed
//! capacity); `push` and the queries never allocate. Only the UI thread owns it; it is fed from
//! `RenderEvent::FrameReady` in `main.rs`. A rail-time wrap moves the current loop curve to the
//! previous-loop slot; a new cassette, Auto-Drive start or the Reset button clears everything.
//! **Used by** `main.rs` (feeding, reset, export) and `ui.rs` (`ui_camera_stats`).

use crate::perf_log::PerfSample;

/// Frames kept for statistics and export (about 4.5 min at 60 fps; older frames drop out).
pub const FLIGHT_CAP: usize = 16_384;
/// Points per loop curve (a 34 s rail at 60 fps is 2040 points).
pub const LOOP_CAP: usize = 4_096;
/// Bins of the RT histogram.
pub const HIST_BINS: usize = 24;
/// Slowest rail positions kept.
pub const SLOW_SPOTS: usize = 5;
/// Two slow frames closer than this on the rail (seconds) are the same spot.
const SLOW_SPOT_MIN_SEP_S: f32 = 1.0;

/// Scene load of one frame (from `CsgStats`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SceneLoad {
    /// Visible CSG instances in the IAS.
    pub visible: u32,
    /// Visible parts at LOD 0 / 1 / 2.
    pub lod: [u32; 3],
    /// Culled instances / parts.
    pub culled: u32,
    /// Visible instances by evaluation path: single primitive, pure-union multi-AABB,
    /// first-hit-union multi-AABB (exact per-part rule), RPN (`CsgStats::path_instances`).
    pub paths: [u32; 4],
}

/// One point of a loop curve.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LoopPoint {
    pub rail_t: f32,
    pub rt_ms: f32,
    pub visible: u32,
    /// LOD 0 parts / all visible parts (`0..=1`).
    pub lod0_share: f32,
}

/// A slow rail position.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SlowSpot {
    pub rt_ms: f32,
    pub rail_t: f32,
    pub eye: [f32; 3],
}

/// Accumulated RT statistics.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RtSummary {
    pub frames: usize,
    pub min: f32,
    pub median: f32,
    pub p95: f32,
    pub max: f32,
}

/// See the module docs.
pub struct FlightStats {
    samples: Vec<PerfSample>,
    head: usize,
    len: usize,
    cur_loop: Vec<LoopPoint>,
    prev_loop: Vec<LoopPoint>,
    slow: [Option<SlowSpot>; SLOW_SPOTS],
    sorted: Vec<f32>,
    last: Option<(f32, [f32; 3], [f32; 3])>,
    /// Camera speed along the rail (metres per rail second).
    pub speed_mps: f32,
    /// Camera view-direction turn rate (degrees per rail second).
    pub turn_dps: f32,
    /// Scene load of the latest frame.
    pub load: SceneLoad,
    /// Completed loops since the last reset.
    pub loops: u32,
}

impl Default for FlightStats {
    fn default() -> Self {
        Self::new()
    }
}

impl FlightStats {
    /// Allocates every buffer once.
    pub fn new() -> Self {
        Self {
            samples: vec![PerfSample::default(); FLIGHT_CAP],
            head: 0,
            len: 0,
            cur_loop: Vec::with_capacity(LOOP_CAP),
            prev_loop: Vec::with_capacity(LOOP_CAP),
            slow: [None; SLOW_SPOTS],
            sorted: Vec::with_capacity(FLIGHT_CAP),
            last: None,
            speed_mps: 0.0,
            turn_dps: 0.0,
            load: SceneLoad::default(),
            loops: 0,
        }
    }

    /// Clears all accumulated data (keeps the allocations).
    pub fn reset(&mut self) {
        self.head = 0;
        self.len = 0;
        self.cur_loop.clear();
        self.prev_loop.clear();
        self.slow = [None; SLOW_SPOTS];
        self.last = None;
        self.speed_mps = 0.0;
        self.turn_dps = 0.0;
        self.load = SceneLoad::default();
        self.loops = 0;
    }

    /// Frames accumulated since the last reset (at most [`FLIGHT_CAP`]).
    pub fn frames(&self) -> usize {
        self.len
    }

    /// Records one rendered frame at rail time `rail_t` of a rail lasting `duration_s`, with the
    /// camera view direction `fwd` (unit) and the frame's scene load.
    pub fn push(
        &mut self,
        perf: &PerfSample,
        rail_t: f32,
        duration_s: f32,
        fwd: [f32; 3],
        load: SceneLoad,
    ) {
        let eye = perf.state.eye;
        if let Some((last_t, last_eye, last_fwd)) = self.last {
            if rail_t + 0.5 * duration_s.max(0.5) < last_t {
                // Rail wrapped: the finished loop becomes the overlay of the next one.
                std::mem::swap(&mut self.cur_loop, &mut self.prev_loop);
                self.cur_loop.clear();
                self.loops = self.loops.saturating_add(1);
            } else {
                let dt = rail_t - last_t;
                if dt > 1e-5 {
                    self.speed_mps = dist(eye, last_eye) / dt;
                    let c = dot(fwd, last_fwd).clamp(-1.0, 1.0);
                    self.turn_dps = c.acos().to_degrees() / dt;
                }
            }
        }
        self.last = Some((rail_t, eye, fwd));
        self.load = load;

        self.samples[self.head] = *perf;
        self.head = (self.head + 1) % FLIGHT_CAP;
        self.len = (self.len + 1).min(FLIGHT_CAP);

        if self.cur_loop.len() < LOOP_CAP {
            let parts = load.lod[0] + load.lod[1] + load.lod[2];
            self.cur_loop.push(LoopPoint {
                rail_t,
                rt_ms: perf.rt_ms,
                visible: load.visible,
                lod0_share: if parts > 0 {
                    load.lod[0] as f32 / parts as f32
                } else {
                    0.0
                },
            });
        }
        self.note_slow(SlowSpot {
            rt_ms: perf.rt_ms,
            rail_t,
            eye,
        });
    }

    fn note_slow(&mut self, spot: SlowSpot) {
        if !spot.rt_ms.is_finite() {
            return;
        }
        // Same spot (close on the rail): keep the slower frame of it.
        if let Some(near) = self
            .slow
            .iter_mut()
            .flatten()
            .find(|s| (s.rail_t - spot.rail_t).abs() < SLOW_SPOT_MIN_SEP_S)
        {
            if spot.rt_ms > near.rt_ms {
                *near = spot;
            }
        } else if let Some(slot) = self.slow.iter_mut().find(|s| s.is_none()) {
            *slot = Some(spot);
        } else if let Some(fastest) = self
            .slow
            .iter_mut()
            .flatten()
            .min_by(|a, b| a.rt_ms.total_cmp(&b.rt_ms))
            && spot.rt_ms > fastest.rt_ms
        {
            *fastest = spot;
        }
        self.slow.sort_by(|a, b| {
            b.map(|s| s.rt_ms)
                .unwrap_or(f32::NEG_INFINITY)
                .total_cmp(&a.map(|s| s.rt_ms).unwrap_or(f32::NEG_INFINITY))
        });
    }

    /// The newest accumulated frame.
    pub fn latest(&self) -> Option<&PerfSample> {
        (self.len > 0).then(|| &self.samples[(self.head + FLIGHT_CAP - 1) % FLIGHT_CAP])
    }

    /// Accumulated frames, oldest first.
    pub fn samples(&self) -> impl Iterator<Item = &PerfSample> {
        let start = (self.head + FLIGHT_CAP - self.len) % FLIGHT_CAP;
        (0..self.len).map(move |i| &self.samples[(start + i) % FLIGHT_CAP])
    }

    /// RT min / median / p95 / max over the accumulated frames (`None` before the first frame).
    pub fn summary(&mut self) -> Option<RtSummary> {
        if self.len == 0 {
            return None;
        }
        let mut sorted = std::mem::take(&mut self.sorted);
        sorted.clear();
        sorted.extend(self.samples().map(|s| s.rt_ms));
        sorted.sort_unstable_by(f32::total_cmp);
        let n = sorted.len();
        let out = RtSummary {
            frames: n,
            min: sorted[0],
            median: sorted[n / 2],
            p95: sorted[((n as f32 * 0.95) as usize).min(n - 1)],
            max: sorted[n - 1],
        };
        self.sorted = sorted;
        Some(out)
    }

    /// Counts accumulated RT values into `bins` equal bins over `[lo, hi]` (out of range values
    /// go to the edge bins).
    pub fn histogram(&self, lo: f32, hi: f32, bins: &mut [u32; HIST_BINS]) {
        *bins = [0; HIST_BINS];
        let span = (hi - lo).max(1e-3);
        for s in self.samples() {
            let f = ((s.rt_ms - lo) / span * HIST_BINS as f32).floor();
            let i = if f.is_finite() {
                f.clamp(0.0, (HIST_BINS - 1) as f32) as usize
            } else {
                0
            };
            bins[i] += 1;
        }
    }

    /// Current loop curve.
    pub fn cur_loop(&self) -> &[LoopPoint] {
        &self.cur_loop
    }

    /// Previous complete loop curve (empty before the first wrap).
    pub fn prev_loop(&self) -> &[LoopPoint] {
        &self.prev_loop
    }

    /// Slowest rail positions, slowest first.
    pub fn slow_spots(&self) -> impl Iterator<Item = &SlowSpot> {
        self.slow.iter().flatten()
    }

    /// Writes the accumulated frames as the perf CSV (same schema as `--bench-rail`).
    pub fn export_csv(&self, path: &std::path::Path) -> Result<(), String> {
        crate::perf_log::write_csv(path, self.samples())
    }
}

fn dist(a: [f32; 3], b: [f32; 3]) -> f32 {
    let d = [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    dot(d, d).sqrt()
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(rt: f32, eye: [f32; 3]) -> PerfSample {
        let mut s = PerfSample {
            rt_ms: rt,
            ..PerfSample::default()
        };
        s.state.eye = eye;
        s
    }

    #[test]
    fn accumulates_summary_speed_and_loop_wrap() {
        let mut f = FlightStats::new();
        assert!(f.summary().is_none());
        let fwd = [0.0, 0.0, -1.0];
        for i in 0..10 {
            let t = i as f32;
            f.push(
                &frame(10.0 + t, [0.0, 0.0, -2.0 * t]),
                t,
                10.0,
                fwd,
                SceneLoad::default(),
            );
        }
        assert!((f.speed_mps - 2.0).abs() < 1e-4, "2 m per rail second");
        assert_eq!(f.turn_dps, 0.0);
        let s = f.summary().expect("frames");
        assert_eq!((s.frames, s.min, s.max), (10, 10.0, 19.0));
        assert_eq!(s.median, 15.0);
        assert_eq!(f.cur_loop().len(), 10);
        // Wrap to the rail start: the loop moves to the overlay slot.
        f.push(&frame(5.0, [0.0; 3]), 0.0, 10.0, fwd, SceneLoad::default());
        assert_eq!(
            (f.loops, f.prev_loop().len(), f.cur_loop().len()),
            (1, 10, 1)
        );
        f.reset();
        assert_eq!((f.frames(), f.loops, f.cur_loop().len()), (0, 0, 0));
    }

    #[test]
    fn keeps_the_five_slowest_separate_spots() {
        let mut f = FlightStats::new();
        let fwd = [0.0, 0.0, -1.0];
        // Two frames 0.1 s apart are one spot; ten spots 2 s apart compete for five slots.
        for i in 0..10 {
            let t = 2.0 * i as f32;
            f.push(
                &frame(i as f32, [0.0; 3]),
                t,
                100.0,
                fwd,
                SceneLoad::default(),
            );
            f.push(
                &frame(i as f32 + 0.5, [0.0; 3]),
                t + 0.1,
                100.0,
                fwd,
                SceneLoad::default(),
            );
        }
        let rts: Vec<f32> = f.slow_spots().map(|s| s.rt_ms).collect();
        assert_eq!(rts, vec![9.5, 8.5, 7.5, 6.5, 5.5]);
    }

    #[test]
    fn ring_keeps_the_newest_frames_in_order_and_histogram_counts_all() {
        let mut f = FlightStats::new();
        let n = FLIGHT_CAP + 7;
        for i in 0..n {
            f.push(
                &frame(i as f32, [0.0; 3]),
                i as f32 * 1e-3,
                1e9,
                [0.0, 0.0, -1.0],
                SceneLoad::default(),
            );
        }
        assert_eq!(f.frames(), FLIGHT_CAP);
        let first = f.samples().next().map(|s| s.rt_ms);
        let last = f.samples().last().map(|s| s.rt_ms);
        assert_eq!(first, Some(7.0));
        assert_eq!(last, Some((n - 1) as f32));
        let mut bins = [0u32; HIST_BINS];
        f.histogram(0.0, n as f32, &mut bins);
        assert_eq!(bins.iter().sum::<u32>() as usize, FLIGHT_CAP);
    }
}
