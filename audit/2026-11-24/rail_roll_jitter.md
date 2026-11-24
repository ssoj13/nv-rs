# Rail roll twitch over the Gundo tanks — root cause

Date: 2026-11-24. Read-only analysis; no cargo runs. Sources: nv-rs working tree, `csg-rs` at the
Cargo.lock revision `fa5e170` (`~/.cargo/git/checkouts/csg-rs-3a6ed3e588aad103/fa5e170`),
`camera-rs` `9109108`. A float64 Python copy of the bank code path was run on synthetic rails
(scratchpad, not in the repo); its numbers are quoted below.

## Symptom

On scene `gundo` the camera tilts left/right by about 0.5 deg for 1-2 frames, always at the same
spot: over the tank farm, on the stretch from the tanks toward the shore near the end of the
rail.

## Where the roll comes from (only one source)

| Step | Code | Effect on roll |
| --- | --- | --- |
| nv-demo evaluates the scene cassette | `crates/nv-demo/src/camera_rail.rs:661-675` | Passes `sample.forward` and `sample.up` straight to `CameraPose::from_eye_forward_up`. |
| camera-rs builds the basis | `camera-rs/crates/core/src/pose.rs:89-119` | `right = f x up`, `true_up = right x f`. A plain orthonormalisation with no state, so it adds no roll and no hemisphere flip. |
| Damper | `camera_rail.rs:751-787`, `main.rs:4919-4926` | `cam_smooth_tau` defaults to `0.0` (`renderer.rs:1039`). `step()` therefore returns the target unchanged (`camera_rail.rs:752`). With tau > 0 it is a first-order filter with no overshoot, so it cannot cause a twitch. |
| Rail sample basis | `rail.rs:771-781` | `up_base` comes from world `+Y`. Then `up = up_base*cos(bank) + right_base*sin(bank)`. Screen roll equals `bank_rad` exactly. Pitch and yaw changes add no roll. |

The twitch is therefore a 1-3 frame spike in `RailSample::bank_rad`. Quaternion sign flips,
look-at target jumps and the damper are ruled out: none of them changes roll on this path.

## Root cause: the bank reads a narrow curvature spike in the eye spline at a single point

`bank_rad` = `compute_smoothed_bank_rad_arc` (`rail.rs:333-341`, `646-674`):

1. Three taps at arc fractions `f - df`, `f`, `f + df`, weighted `0.25/0.5/0.25`.
   `df = (5 / total_len).clamp(0.005, 0.025)` (`rail.rs:304`). On a rail of about 1.9 km this
   puts the taps about 9.5 m apart.
2. Each tap is `compute_bank_rad_single` (`rail.rs:569-611`): a 3-point second difference
   `accel = (v2 - v1) / ds^2` with a fixed step of **`du = 0.025` in spline parameter `u`**. It is
   projected on `right_flat = normalize(t x Y)` and mapped through
   `atan(kappa_lat * bank_gain) * 2/pi * max_bank`.

Three facts combine:

- **Pointwise sampling.** Each segment spans exactly 1.0 in `u` (`segment_indices`,
  `rail.rs:475-505`). The stencil therefore covers `0.05 * segment_length` of arc: 0.2 m on a 4 m
  segment, 3 m on a 60 m segment. The "Gaussian" is three point samples about 9.5 m apart, not an
  average. A curvature feature narrower than the camera's travel per frame (about 55 m/s / 60 =
  0.9 m) goes into `bank_rad` unattenuated, at 25 %, 50 % and 25 %.
- **The spline has such spikes.** The rail uses centripetal Catmull-Rom (`rail.rs:507-529`), which
  is only C1. Where a short segment sits between long ones, the knot tangents come from very
  uneven chords. The curve then makes a millimetre-sized lateral S inside the short segment, with
  very high curvature. In the copy, **all waypoints lie exactly on an R = 300 m circle** (true
  kappa*R = 1). Inserting a single extra point 4 m after a knot gives a true spline kappa*R of
  **32.7** inside that 4 m segment.
- **The gain is high and there is no dead-zone.** For GrandTour/UrbanCanyon (`12 deg, gain 18`,
  `rail_gen.rs:88-95`), 0.5 deg of bank needs only kappa ~ 0.0036 1/m (radius ~ 280 m). The
  lateral wiggle is invisible in position (mm), but the bank turns it into degrees.

Output of the copy at 60 fps, 34 s loop, about 55 m/s:

| Rail | max abs(d bank / frame) | bank range |
| --- | --- | --- |
| 24 even points on an R = 300 m circle | 0.001 deg | 0.45..0.47 deg (correct) |
| same + one point 4 m after waypoint 5 (still on the circle) | **3.05 deg** | -0.10..4.34 deg |
| same + 6 m vertical lift on 2 waypoints (tank escape) | 1.35 deg | -0.72..1.90 deg |

Per-frame trace of the second rail: bank stays at 0.18-0.20 deg, then jumps
`0.16 -> 1.09 -> 2.25 -> 0.58 -> 0.03` (frames 415-419, `f + df` tap),
`0.07 -> 1.29 -> 4.34 -> 1.62 -> -0.10` (frames 425-429, centre tap) and
`0.10 -> 0.49 -> 2.13 -> 1.34 -> 0.10` (frames 435-439, `f - df` tap). That is three separate
twitches, each 2-3 frames long, about 10 frames apart (df / v), with the side flipping between them.
This matches "a 1-2 frame left/right tilt" (Gundo's real segment ratio is milder, hence about
0.5 deg instead of 2-4 deg).

## Why it happens over the tanks

`relax_rail_against_world` (`rail_gen.rs:530-820`) produces exactly the uneven, short segments
described above wherever samples hit solids, and the tank farm is the densest hit zone on the
Gundo rail:

- Tanks are cylinders of radius 11.5 m and 9.2 m, in 3-14 rows x 2 columns, 30 m apart
  (`csg_scene_gundo.rs:374-392, 883-899`). A rail crossing them violates clearance for many
  consecutive samples.
- An inside-solid hit pushes along `hit.normal * 0.5` horizontally (`rail_gen.rs:619-623`). On a
  cylinder that normal is radial, so it is a **lateral** push that differs from waypoint to
  waypoint (clamped to 1.8 m, `:722-731`). The escape phase also lifts flagged waypoints by
  2..14 m (`:734-742`).
- Laplacian smoothing skips hit waypoints (`:758-760`), so these lateral and vertical kinks stay.
- At iterations 7 and 13, a waypoint is inserted at the worst violation (`:780-802`), offset
  `clearance*1.25` along the push direction. That splits a long segment and creates the short/long
  ratio that produces the curvature spike.
- The seed polyline adds bevel triplets of ~5 m spacing at turns > 40 deg (`bevel_sharp_corners`
  `:464-523`, via `finalize_closed_polyline` `:2544-2561`). For a non-street leg it also adds a
  lifted, bowed mid-arc point (`build_tour_polyline` `:2522-2537`). Both again put short segments
  next to long ones.

On "end of the rail": every generated rail is `looping = true` (`rail_gen.rs:2597`). The
tanks -> shore stretch is most likely the closing leg (last tour spot -> spot 0). I checked the
closure math and it is periodic and consistent:
- `sanitize_frac` uses `rem_euclid` (`rail.rs:398-407`).
- `segment_indices` wraps `u` modulo N with wrapped neighbour indices (`:484-494`).
- The LUT ends exactly at `u = N`, which evaluates to waypoint 0 (`:152-174, 194-207`).
- Target look-ahead and occlusion steering wrap too (`rail_gen.rs:863-877, 987-996`).
- `CameraRail::tick` / `evaluate_at` wrap time with `rem_euclid` (`camera_rail.rs:634-635, 650-651`).

The seam itself adds no discontinuity. The closing leg only happens to be where the relaxer
crowded the tank-escape waypoints.

**Exact parameter:** cannot be derived without building the Gundo scene (forbidden while the GPU
profiling runs). Test 2 below prints it (frac, `u`, segment index, segment lengths).

## Proposed fix (systemic, in `csg-scene/src/rail.rs`)

Compute bank from the **heading turn rate over a fixed arc-length window** instead of the
point-sampled second difference in `u`. Integrating curvature over a window turns a spike of
width w into its integral (a near-zero heading change), so it can no longer get through.

```rust
/// Arc-length window over which the bank's lateral curvature is measured (meters).
const BANK_WINDOW_M: f32 = 12.0;
/// Turn rates below this (1/m, radius 2 km) are treated as straight flight: no micro-bank.
const BANK_DEAD_ZONE: f32 = 5.0e-4;

fn heading_xz(wps: &[RailWaypoint], lut: &ArcLengthLut, looping: bool, f: f32, h: f32) -> Option<f32> {
    let p0 = eval_eye_at_u(wps, looping, lut.u_for_fraction(sanitize_frac(f - h, looping)));
    let p1 = eval_eye_at_u(wps, looping, lut.u_for_fraction(sanitize_frac(f + h, looping)));
    let d = p1 - p0;
    (d.x.hypot(d.z) > 1e-4).then(|| d.x.atan2(d.z))
}

/// Bank from the horizontal heading change across `±BANK_WINDOW_M` of arc length.
fn bank_from_turn_rate(wps: &[RailWaypoint], lut: &ArcLengthLut, looping: bool, f: f32,
                       max_bank_deg: f32, bank_gain: f32) -> f32 {
    let total = lut.total_length();
    if !(total > 1.0) || !max_bank_deg.is_finite() || !bank_gain.is_finite() { return 0.0; }
    let h = BANK_WINDOW_M / total;
    let (Some(a), Some(b)) = (heading_xz(wps, lut, looping, f - h, 0.5 * h),
                              heading_xz(wps, lut, looping, f + h, 0.5 * h)) else { return 0.0 };
    let dpsi = (a - b + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
    let kappa = dpsi / (2.0 * BANK_WINDOW_M);
    let kappa = kappa.signum() * (kappa.abs() - BANK_DEAD_ZONE).max(0.0);
    let max_rad = max_bank_deg.abs().min(MAX_CINEMA_BANK_DEG).to_radians();
    ((kappa * bank_gain).atan() * std::f32::consts::FRAC_2_PI * max_rad).clamp(-max_rad, max_rad)
}
```

- Replace `compute_smoothed_bank_rad_arc` in `eval_at_fraction` with this function (an optional
  5-tap Gaussian at `f ± k*h` widens it further).
- Route `eval_at_u` through `lut.fraction_for_u(u)` so the two entry points share one bank
  definition. Delete `compute_smoothed_bank_rad`, `compute_bank_rad_single` and their fixed `du`
  constants.
- Sign: in the copy the replacement gives the same sign and value as the current code on a clean
  circle (0.45..0.47 deg both). Confirm with the existing `rail_c1_periodic_seam_continuity_and_banking`
  (`csg_scene.rs:422-477`) and the 14-deg cap test (`:950-956`).
- Measured in the copy: the "circle + 4 m segment" rail goes from 3.05 deg to **0.077 deg** max
  per-frame change; the tank-lift variant goes from 1.35 deg to 0.13 deg. Clean rails are
  unchanged.
- Cost: 4 eye evaluations instead of 9 per frame.

Optional second step (generator): after relaxation, cap the ratio between adjacent segment
lengths at about 3:1 by resampling the eye polyline. That also removes the mm-scale position S
from the spline. It is not needed for the roll fix.

Not recommended:
- Damping bank in nv-demo, or a non-zero `cam_smooth_tau`. It only hides the spike, adds lag to
  every rail and leaves `RailSample` wrong for other users (SDG export, ViewProbe).
- Raising `du`. The window would still scale with segment length in `u`, which is the defect.

## Proposed regression tests (not added to the repo)

Test 1: synthetic, in `crates/csg-scene/src/csg_scene.rs` tests. It fails on `fa5e170` (copy:
3.05 deg/frame) and passes with the fix (0.077).

```rust
#[test]
fn rail_bank_has_no_twitch_at_short_segments() {
    use crate::rail::{RailSpec, RailWaypoint};
    use csg_core::Vec3;
    let r = 300.0f32;
    let mut angles: Vec<f32> = (0..24).map(|i| std::f32::consts::TAU * i as f32 / 24.0).collect();
    // One extra waypoint 4 m after waypoint 5, still exactly on the circle (short/long knot).
    angles.insert(6, angles[5] + 4.0 / r);
    let wps: Vec<RailWaypoint> = angles.iter().map(|&a| {
        let eye = Vec3::new(r * a.cos(), 20.0, r * a.sin());
        RailWaypoint::new(eye, Vec3::new(0.0, 20.0, 0.0), 50.0)
    }).collect();
    let rail = RailSpec::new("twitch", "twitch", "", wps, 34.0)
        .with_banking(12.0, 18.0).with_looping(true).compile();
    let frames = (34.0 * 60.0) as usize;
    let mut prev = rail.eval_at_fraction(0.0).bank_rad;
    let mut worst = (0.0f32, 0usize);
    for k in 1..=frames {
        let b = rail.eval_at_fraction(k as f32 / frames as f32).bank_rad;
        let d = (b - prev).abs().to_degrees();
        if d > worst.0 { worst = (d, k); }
        prev = b;
    }
    assert!(worst.0 < 0.15, "bank twitch {:.3} deg/frame at frame {}", worst.0, worst.1);
}
```

Test 2: Gundo locator and gate, in `scenes/csg-scene-gundo` tests. It runs on the real rails and
prints the exact twitch location (frac, `u`, segment, segment lengths).

```rust
#[test]
fn gundo_rails_bank_is_twitch_free_at_60fps() {
    let built = GundoScene.build(&SceneBuildConfig::default()).expect("Gundo builds");
    for spec in &built.rails {
        let rail = spec.compile();
        let frames = (spec.duration_s * 60.0).round() as usize;
        let n = spec.waypoints.len();
        let mut b = Vec::with_capacity(frames + 1);
        for k in 0..=frames { b.push(rail.eval_at_fraction(k as f32 / frames as f32).bank_rad); }
        // Second difference catches a twitch (out-and-back) and not a legitimate bank ramp.
        let (mut worst, mut at) = (0.0f32, 0usize);
        for k in 1..frames {
            let d2 = (b[k + 1] - 2.0 * b[k] + b[k - 1]).abs().to_degrees();
            if d2 > worst { worst = d2; at = k; }
        }
        let frac = at as f32 / frames as f32;
        let u = rail.lut.u_for_fraction(frac);
        let seg = (u.rem_euclid(n as f32)).floor() as usize % n;
        let len = |i: usize| (spec.waypoints[(i + 1) % n].eye - spec.waypoints[i].eye).len();
        println!("{}: worst |d2 bank| {:.3} deg at frame {at} frac {frac:.4} u {u:.3} seg {seg} \
                  len[prev,cur,next]=[{:.1},{:.1},{:.1}] m", spec.id, worst,
                 len((seg + n - 1) % n), len(seg), len((seg + 1) % n));
        assert!(worst < 0.10, "{}: bank twitch {:.3} deg at frac {frac:.4}", spec.id, worst);
    }
}
```

Note: `rail.lut` is a public field, but `u_for_fraction` is `pub`, so the test compiles from the
scene crate. nv-demo may build Gundo with a seed other than `SceneBuildConfig::default()`. If the
default does not reproduce, run the test with the seed nv-demo passes (`csg_scene_host.rs`).
