# nv-demo slice audit: camera_core / camera_egui / camera_rail / demo_anim / mp4_muxer

Date: 2026-11-23. HEAD `38ec140`. Read-only; no cargo run.

Scope note: the brief expected separate view-probe / Smart View / Smart Rail / possess-body / sensor files. In this slice there are none. That logic lives in excluded files (`main.rs`, `renderer.rs`, `csg_scene_host.rs`). The slice is `camera_core/**` (a vendored fork of `cglibs/camera-rs`), `camera_egui.rs`, `camera_rail.rs`, `demo_anim.rs`, `mp4_muxer.rs` and `Cargo.toml`. There are no `tests/`, `benches/` or `examples/` directories.

## 1) Item-by-item ledger

| ID | Sev | Verdict | Location | Defect | Failing scenario | Fix direction |
|---|---|---|---|---|---|---|
| D01 | P1 | CONFIRMED | `mp4_muxer.rs:53-102`, `:210-238` (no `impl Drop`) | The output is written in place (`File::create(path)`, not atomic). The `mdat` largesize placeholder is `16` (`:81`), and `moov` exists only after a successful `finish()`. `Mp4Muxer` has no `Drop`. | Any path that skips a successful `finish()` leaves a corrupt file whose `mdat` header claims 16 bytes and that has no `moov`: `finish()` returning `Err("missing H.264 SPS for avcC")`, an export-worker panic, process kill, or OS crash. The file cannot be played. Box-walking recovery tools also fail, because the next "box" is parsed from inside NAL data. | Write to `<path>.partial` in the same directory and rename it after `finish` succeeds (flush plus `sync_all`). Use largesize `0` ("extends to EOF") as the placeholder so a crash leaves a recoverable file. Add a best-effort `Drop` that finalizes the file or deletes the partial. |
| D02 | P1 | CONFIRMED | `mp4_muxer.rs:211-217`; caller `main.rs:1520-1523` | `finish()` with 0 samples flushes a 32-byte `ftyp` + `mdat` stub and returns `Ok(0)`. | Start recording and stop before the first encoded sample (or every AU is dropped, see D18). An unplayable 32-byte `.mp4` stays on disk and the UI says `"Saved MP4 (0 frames)"`. | Return `Err` (or delete the file) when `count == 0`. |
| D03 | P1 | PLAUSIBLE | `camera_core/input/raw.rs:449-453` (`RawInput::sanitized`) | `dt` is passed through uncapped. Upstream camera-rs (`crates/core/src/input/raw.rs`) added `MAX_DT = 0.1` for exactly this. egui `stable_dt` equals `unstable_dt` uncapped when the previous frame requested a repaint (`egui-0.36.2/src/input_state/mod.rs:375-383`). | A main-thread stall of 2 s (scene switch, modal) while WASD is held during flight: `Drive::step` (`controller/thrust.rs:94-104`) integrates `target*dt`. The camera jumps `2 s * cruise * speed_scale`, where cruise is up to 96 u/s at altitude ≥ 200 (`fly.rs:62-67`), times Shift ×4 × ramp up to 64 (`fly.rs:49-55`) × wheel up to 64 (`fly.rs:42`). | Port `MAX_DT` into `sanitized()`. |
| D04 | P2 | PLAUSIBLE | `controller/quat.rs:176-216`; `rig/orbit.rs:621-686` | Coast `kick` adds the new release velocity to the running coast velocity with no bound. Upstream clamps with `clamp_coast_speed(MAX_ANGULAR/PAN/ZOOM/DOLLY_ZOOM_COAST_RATE)`. | Repeated quick flick-releases on a turntable or trackball compound the turn and pan coasts geometrically. The view spins very fast, or the pan coast flings the pivot hundreds of distances away in a city scene. | Port upstream `clamp_coast_speed` and its constants. |
| D05 | P2 | CONFIRMED | `main.rs:7-10` (the suppression is in an excluded file but covers this slice); 36 `cfg(feature = "serde")` sites in `camera_core` (e.g. `coast.rs:73-77`, `prefs.rs:110-114`, `settings.rs:24-37`, `pose.rs:59`) | `#[allow(unexpected_cfgs, dead_code, unused_imports, clippy::all)] mod camera_core;` and the same plus `deprecated` on `camera_egui`. `nv-demo/Cargo.toml` declares no `[features]`, so every `cfg_attr(feature = "serde", ...)` is dead. The 5 serde tests never compile, so they are silently skipped: `mode.rs:405`, `prefs.rs:589`, `prefs.rs:619`, `settings.rs:315`, `settings.rs:338`. I found no `#[deprecated]` egui 0.36.2 API used by `camera_egui.rs` (checked the registry source), so the `deprecated` allow looks stale. `clippy::all` is silenced for about 10k LOC. | `cargo clippy -D warnings` cannot see this code. Any future warning (an unfinished feature, a swallowed result) is hidden. | Remove the `cfg_attr` serde lines (or declare a `serde` feature), delete the dead items (D06), and drop the blanket `allow`s. Use `#[expect(..., reason)]` only where justified. |
| D06 | P2 | CONFIRMED | `auto_orbit.rs` (all); `driver.rs:377-390`, `:484-518` (lens); `driver.rs:312-323` (`set_view_roll`); `camera_egui.rs:227-232` (`set_raw_motion_units`); `viewport.rs:152-208` (`TextureUv`, `ScreenRect`); `view.rs:306-343` (`orbit_about`, `roll_about`); `lens.rs:128-206` (focal/zoom helpers) | Dead vendored API that nothing outside `camera_core` constructs or calls (grep across `src/`). `CameraDriver::set_lens` is never called, so `step_lens` always returns `false`. `set_view_roll` is never called, so `roll_intent` is always the identity. | None at runtime. Dead weight hidden by D05. | Delete these, or consume the upstream crate (D07). |
| D07 | P2 | CONFIRMED | `camera_core/**`, `camera_egui.rs` versus `cglibs/camera-rs/crates/{core,egui}/src` @ `ab89440` | A diverged fork, so there is no single source of truth. Changed-line counts per file: `input/fly.rs` 237, `driver.rs` 130, `coast.rs` 130, `controller/space.rs` 116, `controller/fps.rs` 114, `controller/thrust.rs` 70, `input/navigator.rs` 62, `prefs.rs` 47, `input/bindings.rs` 41, `rig/orbit.rs` 38, `controller/quat.rs` 37, `camera_egui.rs` 302. Upstream fixes missing here: D03, D04, D24, the `Studio` scheme, `Bindings::without_app_keys`. The `invert_look_x` default differs (`settings.rs:246` `true` here, upstream `false`). nv-demo-only additions: `shift_fly_ramp`, `altitude_speed_multiplier`, `FLY_WHEEL_STEP` 1.25, `FLY_SPEED_MIN` 0.1. Doc drift: the module docs still describe standalone crates `camera-core` / `camera-egui` ("wasm-clean", "feature serde", "Used by the camera umbrella", "egui-widgets-rs, nodes-egui hosts"): `mod.rs:1-14`, `camera_egui.rs:1-11`, `:73`, `driver.rs:47-48`. | Bug fixes land in one copy only. | Depend on camera-rs through a git ref (cglibs policy). Upstream the two speed helpers (or move them into nv-demo host code). |
| D08 | P2 | PLAUSIBLE | `input/fly.rs:62-67`; callers `main.rs:3972-3975`, `ui.rs:1094`, `ui.rs:1244`; `driver.rs:422-437` | `altitude_speed_multiplier(cam_pose.eye.y)` uses absolute world Y, not height above ground. The result is passed as `scene_radius`. That is part of the driver's `Applied` cache key, so `apply_prefs` and its `sanitized()` calls re-run every frame while the altitude changes. | In a scene whose ground is not at y=0 (DEM terrain, elevated deck), flying 2 m above the ground at world y=150 runs at 18.75× speed. Near-ground inspection becomes impossible. A scene below y=0 always gets 1×. | Pass height above ground from the scene host (ground ray / DEM sample). Apply the multiplier as a speed scale instead of mutating the prefs key. |
| D09 | P2 | CONFIRMED | `camera_rail.rs:52`, `:71`, `:90`, `:657`, `:712` | The FOV clamp `[10°, 120°]` is hand-copied 5 times. The controller range is `lens::FOV_MIN..FOV_MAX = [1°, 120°]` (`lens.rs:97-100`); dolly-zoom reaches 1°. | Dolly-zoom (Ctrl+Alt+LMB) to 5°, then add a keyframe: the keyframe stores 10°, so the Custom rail replays with a different framing. | Use one constant (`lens::FOV_MIN` / `FOV_MAX`) or a single `RAIL_FOV_RANGE`. |
| D10 | P2 | PLAUSIBLE | `camera_rail.rs:447-454` versus `:455-460` and `:521-531` | `set_scene_rails`, when it finds the persisted id, sets only `duration_s`. It keeps the previous `looping`, `ease_io` and `time_s`. `select_scene_rail` and the first-rail fallback reset these from the spec. | Restart, or switch to a scene that exports a rail with the same id: the rail plays with a stale `looping` / `ease_io` from the earlier scene or session and resumes at the old `time_s`. | One `activate(idx)` helper used by all three paths. |
| D11 | P2 | CONFIRMED | `camera_rail.rs:381`, `:396-400`; `demo_anim.rs:17-27`; caller `main.rs:2620-2643` | The derived `PartialEq` includes serde-skipped runtime fields (`CameraRail::playing`, `time_s`; `DemoAnimator::time_s`). `save_persisted` compares against the last saved state every 1.5 s and clones `CameraRail` including `scene.rails: Vec<CompiledRail>`. | With the demo animator running and the camera still, the state is "dirty" every 1.5 s and an identical JSON is enqueued to disk each time. The runtime rail vector is cloned every 1.5 s. | Persist a DTO of the serialized fields only, or write `PartialEq` by hand to ignore runtime fields. |
| D12 | P2 | CONFIRMED | `demo_anim.rs:115-217` versus `ui.rs` `anim_slider_f32` / `anim_vec3` call sites | Stringly-typed keys, with `_ => {}` silently ignoring unknown keys. The 87 `(min, max)` ranges duplicate the UI slider ranges (all currently equal, checked by script). The `"spin_speed"` arm has no UI toggle (`ui.rs:775` is a plain `DragValue`), so only a persisted `active` set can reach it. | A future slider range change, or a key typo, silently desynchronizes animation and UI or disables the animation. | One table (key enum → label, range, field accessor) shared by the UI and the animator. |
| D13 | P2 | PLAUSIBLE | `camera_rail.rs:746-765` (`CameraDamper::step`) | The settle threshold `|eye - target|² < 1e-10` (1e-5 units) is below f32 spacing once `|eye| > ~128` (ulp ≈ 6e-5 at 1e3, 1e-3 at 1e4). `lerp` stops making progress first. | In a km-scale scene (OSM city) the "snap onto the exact target" branch is never taken. The smoothed pose stays 1-2 ulp off the target pose forever, so the documented "settles cleanly" contract fails. Impact on accumulation depends on renderer change detection. | Snap when the step makes no progress (`eye == prev.eye`), or use an epsilon relative to `|target|`. |
| D14 | P2 | CONFIRMED | `mp4_muxer.rs:111`, `:117`, `:319-350` | Per-frame heap allocations: the `split_annexb_nals` Vec and the `sample_payload` Vec, on every recorded frame. | Recording at 60 fps allocates and frees about 120 buffers per second, against the zero-churn policy. | Keep reusable `Vec`s in the struct (`clear()` plus `extend`). |
| D15 | P2 | CONFIRMED | `mp4_muxer.rs:241-244` | `u32::try_from(total_duration_ticks).unwrap_or(u32::MAX)` silently saturates. mvhd/tkhd/mdhd are always version 0. | A recording longer than 47 721 s (about 13.25 h at 90 kHz) gets a wrong container duration. | Write version-1 (64-bit) boxes when the duration exceeds u32. |
| D16 | P2 | PLAUSIBLE | `mp4_muxer.rs:543-560` | The `avcC` for High-family profiles (`profile_idc` 100/110/122/144) omits the `chroma_format` / `bit_depth_luma` / `bit_depth_chroma` / `numOfSequenceParameterSetExt` trailer required by ISO/IEC 14496-15. | Strict demuxers and validators may reject or warn on the SDR H.264 file. | Append `0xFC|1, 0xF8|0, 0xF8|0, 0` (or parse the values from the SPS) when the profile needs them. |
| D17 | P2 | CONFIRMED | `mp4_muxer.rs:518` versus `:531`; `:535`, `:616` | The doc says the `mdcv` minimum is 0.0001 cd/m², but the code writes `50` (0.005 cd/m²). MaxCLL/MaxFALL are hardcoded to 1000/400 regardless of content. | The HDR10 static metadata does not describe the encoded content. Displays tone-map against fabricated values. | Fix the doc. Take MaxCLL/MaxFALL from the tonemapper or the caller. |
| D18 | P2 | PLAUSIBLE | `mp4_muxer.rs:110-114`, `:181-183` | An access unit with no slice NAL (empty bitstream, or only SPS/PPS/AUD) returns `Ok(())` and drops its `duration_ticks`. | If NVENC returns an empty or param-set-only output for a frame (lookahead / latency), those frames' durations vanish. The video is shorter than the recorded time and nothing reports it. | Carry the dropped duration into the next sample, or return a typed error. |
| D19 | P2 | CONFIRMED | `mp4_muxer.rs:693-756` (tests) | The muxer tests only check that fourcc bytes are present (`windows(4).any`). They do not parse box sizes, `stsz`/`co64` against the written payload, `stss` indices, or the patched `mdat` largesize. Fixed temp file names collide across concurrent `cargo test` runs. | A wrong `make_box` size, chunk offset or `mdat` patch would pass every test. | Parse the produced file (walk boxes, check `co64[i] + stsz[i]` lies inside `mdat`). Use unique temp paths. |
| D20 | P2 | PLAUSIBLE | `input/fly.rs:68-82`, `:240-250`; `input/navigator.rs:406-410` | Relative mouse look emits `LookInertia` after a fly drag ends, and after `LOOK_PAUSE_S` in the persistent mode. Upstream removed this because "a coast would overshoot past where the hand stopped". | RMB hold-to-fly under `NavFeel::Smooth`: releasing after a fast look keeps rotating the view past the point where the hand stopped. | Port the upstream change: Look only, with the `look_smoothing` filter ending on the exact pointer total. |

Clean, no findings (read and traced): `num.rs`, `convention.rs`, `projection.rs`, `fit.rs`, `pose.rs`, `orientation.rs` (pole/step rule correct), `dynamics.rs` (`rotate_view` pole STEP rule; `TransitionState` uses glam's shortest-path slerp), `coast.rs` (closed form; `elapsed / time` is only read while `time > 0`), `controller/thrust.rs`, `controller/lock.rs` (`total_cmp`, no unwrap panics), `controller/mod.rs`, `controller/orbit.rs`, `controller/facade.rs`, `controller/space.rs`, `controller/fps.rs` (unbounded `yaw` accumulation noted but harmless), `input/navigator.rs` (apart from D20), `input/bindings.rs`, `intent.rs`, `mode.rs`, `prefs.rs`, `settings.rs`, `view.rs`, `viewport.rs`, `lens.rs`, `camera_egui.rs` (cursor ownership, held-key pruning and raw motion are all correct; the problems are D05/D07 doc and suppression only).

Camera rail spline math, verified correct:
- Centripetal Catmull-Rom knots are floored at 1e-3, so coincident keyframes do not produce NaN.
- Periodic seam indexing `(seg+n-1)%n .. (seg+2)%n` is correct.
- Open-end phantom points `2p1-p2` / `2p2-p1` are correct.
- The arc-length LUT handles zero total length (`u_for_fraction` returns `f`), flat segments (`seg_t = 0`) and the end fractions.
- SQUAD hemisphere alignment (q0/q2 to q1, q3 to q2, tangents re-aligned, log taken with `w ≥ 0`) is correct.
- 0 / 1 / 2 keyframe cases are handled. `shutter_subframe_dt` sums exactly to `frame_dt`.

## 2) Files inspected (manifest: 38 files, 19 848 lines)

Production code was read in full for every file. "Tests" says whether the `#[cfg(test)]` module was read.

| # | File | Lines | Production | Tests |
|---|---|---|---|---|
| 1 | Cargo.toml | 52 | full | n/a |
| 2 | src/camera_egui.rs | 1266 | full | full |
| 3 | src/camera_rail.rs | 995 | full | full |
| 4 | src/demo_anim.rs | 316 | full | full |
| 5 | src/mp4_muxer.rs | 757 | full | full |
| 6 | src/camera_core/mod.rs | 60 | full | n/a |
| 7 | src/camera_core/auto_orbit.rs | 329 | full (1-116) | not read |
| 8 | src/camera_core/coast.rs | 790 | full (1-463) | not read |
| 9 | src/camera_core/controller/facade.rs | 1088 | full (1-612) | not read |
| 10 | src/camera_core/controller/fps.rs | 737 | full (1-391) | not read |
| 11 | src/camera_core/controller/lock.rs | 261 | full | n/a |
| 12 | src/camera_core/controller/mod.rs | 448 | full (1-246) | not read |
| 13 | src/camera_core/controller/orbit.rs | 163 | full (1-143) | not read |
| 14 | src/camera_core/controller/quat.rs | 874 | full (1-342) | not read |
| 15 | src/camera_core/controller/space.rs | 1454 | full (1-706) | partial (707-1222); names only for 1223-1454 |
| 16 | src/camera_core/controller/thrust.rs | 315 | full (1-148) | not read |
| 17 | src/camera_core/convention.rs | 17 | full | n/a |
| 18 | src/camera_core/driver.rs | 1039 | full (1-520) | upstream diff only |
| 19 | src/camera_core/dynamics.rs | 356 | full (1-186) | not read |
| 20 | src/camera_core/fit.rs | 149 | full (1-77) | not read |
| 21 | src/camera_core/input/bindings.rs | 366 | full (doc comments skipped) | n/a |
| 22 | src/camera_core/input/fly.rs | 520 | full (1-353) | test names only |
| 23 | src/camera_core/input/mod.rs | 99 | full (last 30 lines read; module docs via upstream diff) | n/a |
| 24 | src/camera_core/input/navigator.rs | 857 | full (1-741) | not read |
| 25 | src/camera_core/input/raw.rs | 533 | full (doc comments skipped) | not read |
| 26 | src/camera_core/intent.rs | 345 | full (doc comments skipped) | not read |
| 27 | src/camera_core/lens.rs | 755 | full (doc comments skipped) | not read |
| 28 | src/camera_core/mode.rs | 647 | full (doc comments skipped) | not read |
| 29 | src/camera_core/num.rs | 32 | full | full |
| 30 | src/camera_core/orientation.rs | 464 | full (1-252) | not read |
| 31 | src/camera_core/pose.rs | 280 | full (1-251) | not read |
| 32 | src/camera_core/prefs.rs | 632 | full (doc comments skipped) | not read |
| 33 | src/camera_core/projection.rs | 67 | full | full |
| 34 | src/camera_core/rig/mod.rs | 7 | full | n/a |
| 35 | src/camera_core/rig/orbit.rs | 1498 | full (1-845) | not read |
| 36 | src/camera_core/settings.rs | 369 | full (doc comments skipped) | not read |
| 37 | src/camera_core/view.rs | 584 | full (doc comments skipped) | not read |
| 38 | src/camera_core/viewport.rs | 327 | full (doc comments skipped) | not read |

I also read `crates/nv-demo/AGENTS.md`, `nv-rs/AGENTS.md`, `crates/AGENTS.md`, and the upstream `camera-rs` files (diffs only).

Excluded files were read only to confirm contracts:
- `main.rs:7-17`, `:1139-1163`, `:1455-1545`, `:2620-2660`, `:2930-2945`, `:3925-3995`, `:4975-4985`
- `ui.rs:1085-1135`, `:1238-1250`, `:4462-4500`
- `gpu_export.rs:1210-1250`
- `persist.rs` (grep)

Gap: about 7.9k lines of camera_core test modules were not read, so I cannot vouch for those tests' strength. The serde tests listed in D05 are confirmed not to compile.

## 3) Files changed

None. The worktree status is unchanged (only the pre-existing untracked `0/`).

## 4) Static and invariant checks performed

- Upstream parity: per-file `diff` of the vendored `camera_core` / `camera_egui` against `cglibs/camera-rs/crates/{core,egui}/src` @ `ab89440`, with the `crate::camera_core::` → `crate::` path rewrite normalized. Gave the changed-line counts in D07 and found the upstream-only fixes behind D03, D04 and D20.
- Lint suppression: located the module-level `allow`s at `main.rs:7-10`. Confirmed `Cargo.toml` has no `[features]`, so there are 36 dead `cfg(feature="serde")` sites. Grepped the egui 0.36.2 registry source for `#[deprecated]` and found none used by `camera_egui.rs`.
- Dead-code sweep: grepped every candidate public item for use outside `camera_core` (D06).
- egui `stable_dt` semantics: read `egui-0.36.2/src/input_state/mod.rs:294-383` to confirm `dt` is uncapped after repaint-requested frames (D03).
- DemoAnimator ↔ UI: a Perl extraction of every `anim_slider_f32` / `anim_vec3` / `anim_toggle_btn` key and range in `ui.rs`, joined with the `demo_anim.rs` arms. 86 UI keys against 87 animator keys; only extra is `spin_speed`; all ranges equal today.
- MP4 byte layout checked by hand:
  - mvhd (100-byte body), tkhd, mdhd, hdlr, vmhd, dref/url, VisualSampleEntry (78 bytes).
  - hvcC bit packing: `0x0F`, `0xFD`, `0xFA`, `0xF000`, `0xFC`.
  - Profile-tier-level offsets `rbsp[3..15]`.
  - mdcv chromaticities: G 8500/39850, B 6550/2300, R 35400/14600, D65 15635/16450.
  - colr nclx: 1/1/1 for SDR, 9/16/9 for PQ.
  - `co64` offsets = `mdat_header_offset + 16 + running`.
  - `stsc` one sample per chunk, which matches `co64` with one entry per sample.
  - `stss` 1-based indices.
  - `make_box` u32 sizes cannot overflow below roughly 3.5e8 samples.
- Spline/quaternion invariants traced by hand: everything listed under "Camera rail spline math" in section 1.
- Panic surface: no `unwrap`/`expect` or unchecked indexing reachable at runtime in production code of the slice. `camera_rail` indexing is bounded by `spline_segment` (n ≥ 2 checked before). `CustomArcLut` uses `partition_point().clamp(1, N)`. `lock.rs` uses `total_cmp`; there are no `partial_cmp().unwrap()` sorts.

## 5) Risks and open seam questions for other slices

1. `gpu_export.rs:1239-1246` (`VideoRecorder::finish`): it calls `flush_eos()` but never drains bitstream output produced by the flush. If NVENC buffers frames (lookahead, async, B-frames), the tail frames are lost before `Mp4Muxer::finish`. Combined with D18, durations may also vanish. This needs an NVENC-side check.
2. `main.rs:1464`, `:1502`, `:1535`: recorder finish errors are only `eprintln!`'d (previous-session, after-error and shutdown paths). With D01 that leaves corrupt files on disk with no user-visible status in two of three paths.
3. `main.rs:2934` sets `cam_fixed_dt = Some(0.0)`, and `main.rs:1139-1163` builds subframe input with `pointer_delta` zeroed. During fixed-fps recording, `ReleaseVelocity::push` ignores `dt = 0` (`coast.rs:426`) and the subframes push only zero steps. So orbit, pan, dolly and look coasts never seed while recording, and navigation feels different from live. `lockstep_subframe_raw_input(None, ..)` produces `keys_allowed = false`. That triggers `CameraDriver::release_in` (stops coasts, ends hold-fly) if the previous update had keys allowed.
4. `ui.rs:1094`, `main.rs:3972`: see D08. Which ground height does the scene host expose (DEM sample)? The altitude multiplier should use it.
5. Ctrl is both the fly-align modifier (`navigator.rs:403`, Smart alignment while flying) and the Pick-to-Possess modifier (Ctrl+Click). `ui.rs` skips `drive` only when `pick_clicked || pick_active`. Is Ctrl held before the click routed to the driver (snapping the view mid-pick in persistent fly modes)?
6. `ui.rs:1118-1140` (excluded): "frame selected" (`F`) aims at the world origin (`-cur.eye.normalize()`), not at a selection or scene bounds, and ignores `NavRequests::home`. The UI owner should confirm this is intended.
7. Renderer accumulation reset versus D13: does the renderer compare the damped pose against the raw controller pose (which would never settle in km-scale scenes) or frame against frame? The renderer slice should confirm.
8. `persist.rs` does not persist `NavPrefs` / `Bindings` (the serde derives are dead, D05). The camera prefs that are persisted come from `CameraState::from_runtime` in excluded code. Confirm no silent loss of the user's nav tuning.
