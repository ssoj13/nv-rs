# Wave: nv-demo follow-ups (2026-11-23)

Base `main` = `c441ac3`. Nothing committed; all changes are in the working tree for the lead. `cuda_crowd.rs` not touched. `csg_scene_host.rs` not touched at all (no edit was needed there).

## Per item

| # | Item | Verdict | Evidence (before) | Change (file:line, after) | Test |
|---|---|---|---|---|---|
| 1 | GPU:D03 classic object motion | CONFIRMED | `raytracer.cu` motion block: `prev_world_pos = hit_pos` for every non-CSG hit; only CSG instances with `MOTION_BLUR` used `csg_prev_xforms`. Fractal TRS and orbiting spheres had camera-only motion. | New launch fields `classic_motion_count` (636) + `classic_motion` (640), `LaunchParams`/`Params` grown 640 -> **648 B** (Rust `assert!` `renderer.rs:158-168`, CUDA `static_assert` `raytracer.cu:78-90`). `ClassicXforms` + `motion_to` (`renderer.rs:186-247`): per classic IAS instance a row-major 3x4 `prev * cur^-1`, identity when the slot id differs (instance list changed) or the current transform is singular. `build_ias_into` writes the classic slot index into instance-id bits 16..24 (`renderer.rs:2413`) and returns the snapshot. Device table `classic_motion_buf` (28 slots, 1344 B). Shader: `raytracer.cu:2261` maps the classic hit with `csg_xform_pt_ldg`. Hue (bits 8..16), slot kind (0..8), Cryptomatte and instance-AOV ids unchanged. | `classic_motion_maps_current_hit_to_previous_pose`, `classic_motion_without_matching_history_is_identity`, `classic_slot_encoding_fits_instance_id`, `launch_params_abi_size_and_offsets_match_cuda` (updated) |
| 2 | GPU:D21 fractal shape in probe / SDG launches | CONFIRMED for SDG, REFUTED for probe | SDG keypoint launch (`trace_sdg_occluder_optix`) used `..LaunchParams::zeroed()` against the live IAS, which holds the fractal when the classic stage is shown -> occlusion rays marched a zero-parameter fractal. The probe launch builds a CSG-only IAS (`score_views_on_rig`, `append_optix_instances` only), so it never reaches the fractal. The ledger's epsilon part was already fixed (`fractal_aux_launch`). | `FractalLaunch::from_settings` (`renderer.rs:2066`) is the single source for the render launch and the SDG launch (`trace_sdg_occluder_optix` / `export_sdg_json_with_backend` take `&FractalLaunch`). | none host-testable beyond compile (shader behaviour); covered by the app run |
| 3 | `prev_camera` during SDG bursts | REFUTED (already correct), rule made explicit | `prev_camera` was already updated only for `publish_frame && active_burst.is_none()` and once per completed burst frame (Wave 3). | `commits_motion_history` (`renderer.rs:1339`) now owns the rule and moves the camera AND the classic poses together (render loop + burst completion `renderer.rs:5378`, `5808`). | `motion_history_commits_only_on_published_non_burst_frames` |
| 4 | RND:R10 sky crossfade slots | CONFIRMED (partially fixed before) | `pack_crossfade` handled only fades where one side reads no parameters; e.g. Nebula(3) -> Day(0) evaluated Nebula with Day's block. | Dead slots reused: `fractal_xform` -> `env_from_lo` (192), `sat_pos` -> `env_from_hi` (384), same offsets, asserted both sides. `EnvSettings::pack_crossfade` returns `(env_params, env_from)` (`renderer.rs:606`); `sky_color_mode` / `sky_elevated` / `sky_unprogress` / `sky_colonies` take an `env` pointer (`raytracer.cu:633,749,837,888`); `sky_color` evaluates the source sky with `env_from` (`raytracer.cu:1089`). | `sky_crossfade_packs_each_sky_with_its_own_parameters` (replaces the old test) |
| 5 | UI anim slider ranges from `anim_range` | CONFIRMED | 88 call sites passed literal ranges; 87 matched the table, `exposure` did not (slider `0.05..=8.0`, animator clamp `0.25..=2.0`, so animating exposure clamped a user value of e.g. 4.0 down to 2.0). | `anim_slider_f32` / `anim_vec3` lost the `range` argument and look up `anim_range(key)`; unknown key -> debug assert + visible error row (`ui.rs:4458`). `EXPOSURE_RANGE` moved to `demo_anim.rs:21` and feeds the table entry, the slider and the toolbar EV box. The `#[expect(too_many_arguments)]` on `anim_slider_f32` is gone (7 args now). | `ui::tests::anim_slider_keys_are_all_animatable` (`ui.rs:4678`, scans every call site); existing `anim_table_is_unique_ordered_and_drives_every_key` |
| 6 | Per-frame synchronous `params_buf` upload | CONFIRMED | `DeviceBuffer::copy_from_host` synchronises the stream (`cuda-core` `device_buffer.rs:988-999`); called every render frame after the IAS/crowd work, i.e. a mid-frame host stall. | `LaunchUpload` (`renderer.rs:262`): one pinned block (`LaunchParams` + classic table) + `Event`; `upload` waits the event (returns at once behind the frame fence), copies, enqueues `copy_from_host_async_unchecked`, records the event; on any enqueue/record failure it drains the stream so the block is never rewritten under an in-flight copy; `Drop` waits the event before the pinned memory is freed; length mismatch is an `Err`, not the cuda-core panic. Used by the render, probe and SDG launches; no `copy_from_host` left in `renderer.rs`. | ABI test; runtime via nv-demo GPU tests + app run |
| 7 | APP:F19 dock layout persistence | CONFIRMED | `persist.rs` docs said the layout is not persisted; `egui_dock` built without `serde`. | `egui_dock` feature `serde`; `DockTab` serde + `ALL`; `PersistedState::dock: Option<serde_json::Value>` (DockState has no `PartialEq`). `dock_state_to_value` (`persist.rs:219`) zeroes node rects (layout output; `Rect::NOTHING` infinities become JSON `null` and cannot be read back - found by the new test) and re-decodes before returning, so an unreadable layout is never written. `dock_state_from_value` (`persist.rs:239`) requires every tab exactly once. Loader section `dock` (`persist.rs:341`): an invalid layout is a load problem, so the whole file is protected like any other unreadable section (F07). `main.rs:3105` restores it, `main.rs:3305` saves it with the existing 1.5 s atomic temp+rename writer. Live check: after the app run `~/.nv-rs/nv-demo.json` has a `dock` section. | `dock_layout_round_trips`, `invalid_dock_section_is_reported_and_dropped` |
| 8 | RND:H13 `InstanceBuffer` staging growth | CONFIRMED | Fixed 256 KiB staging (3,276 instances); larger payloads (full CSG repack, CPU-crowd prefix of 10-15k bones) took the pageable `cuMemcpyHtoDAsync` path (host-blocking). | `optix/src/accel.rs:1021` `ensure_staging`: grows to the next power of two, capped at `INSTANCE_STAGING_MAX_BYTES` = 64 MiB (`accel.rs:907`); the new buffer is allocated first, the old one is dropped by replacement, and its `Drop` waits its `done` event (fence-before-reuse/free preserved). A failed growth keeps the old buffer, records `staging_error` and is not retried. New `staging_capacity_bytes()` (`accel.rs:1015`). | GPU: `instance_buffer_staging_grows_for_large_uploads` (`optix/tests/gpu_features.rs:3693`): 640 KiB upload grows staging; a queued small update + a 1.3 MB upload that forces a second growth while the stream is held land intact |
| 9 | `nv-wgpu` `submit_vulkan_commands` unbounded wait | CONFIRMED | `nv_wgpu.rs` polled with `timeout: None`. | `VULKAN_SUBMIT_TIMEOUT` = 5 s (`nv_wgpu.rs:69`, > 2 s Windows TDR); `PollError::Timeout` -> typed `Error::Timeout { what }` via `submit_wait_error` (`nv_wgpu.rs:73`). Doc states that after a timeout the GPU must be treated as lost. | `submit_wait_timeout_is_typed` |

## Public API changes
- `optix`: new `INSTANCE_STAGING_MAX_BYTES` (re-exported), new `InstanceBuffer::staging_capacity_bytes()`; `staging_error()` doc now also covers failed growth.
- `nv-wgpu`: new `VULKAN_SUBMIT_TIMEOUT`; `submit_vulkan_commands` can now return `Error::Timeout` (signature unchanged).
- `nv-demo` (binary, crate-internal): `LaunchParams`/`Params` 648 B; `EnvSettings::pack_crossfade` returns a tuple; `demo_anim::EXPOSURE_RANGE` (moved from `ui`); `persist::{dock_state_to_value, dock_state_from_value}`, `PersistedState::dock`; `DockTab` serde.
- Dependencies: `egui_dock` feature `serde` enables `egui/serde`; `Cargo.lock` gains `serde` edges and one new crate, `enumn 0.1.14` (accesskit serde).

## Gate results (this machine, RTX 3080 Ti)
- `cargo clippy --workspace --all-targets --message-format=short -- -D warnings`: `Finished` (only the pre-existing `nvcomp-sys` prebuilt-bindings build warning). First run flagged `needless_range_loop` in `ClassicXforms::motion_to`; fixed.
- `cargo test --workspace --quiet`: **475 passed / 0 failed / 206 ignored** (baseline 467 / 0 / 205; +8 tests, +1 ignored GPU test). First run failed `persist::tests::dock_layout_round_trips` (`invalid type: null, expected f32` - the `Rect::NOTHING` issue in item 7); fixed at the root in `dock_state_to_value`.
- `cargo test -p nv-demo -- --ignored --test-threads=1`: 5 passed / 0 failed.
- `cargo test -p optix -- --ignored --test-threads=1`: 11 + 57 passed / 0 failed (incl. new `instance_buffer_staging_grows_for_large_uploads`).
- `cargo test -p nv-wgpu -- --ignored --test-threads=1`: 13 passed / 0 failed.
- `cargo build -p nv-demo --release`: Finished (nvcc compiled `raytracer.cu` with the new `Params` and `static_assert`s; no build warning).
- `target\release\nv-demo.exe` run > 60 s: process alive (~800 MB working set), stdout/stderr logs empty (`scratchpad\nvdemo_out.log`, `nvdemo_err.log`), config written with the new `dock` section. The screenshot (`scratchpad\nvdemo_followups.png`) only shows terminals: the app window was covered and could not be raised without input automation, so the picture was NOT visually checked. Process was stopped by PID.

## `git status --short`
```
 M AGENTS.md
 M Cargo.lock
 M DIAGRAMS.md
 M README.md
 M crates/nv-demo/AGENTS.md
 M crates/nv-demo/Cargo.toml
 M crates/nv-demo/shaders/raytracer.cu
 M crates/nv-demo/src/demo_anim.rs
 M crates/nv-demo/src/main.rs
 M crates/nv-demo/src/persist.rs
 M crates/nv-demo/src/renderer.rs
 M crates/nv-demo/src/ui.rs
 M crates/nv-wgpu/src/nv_wgpu.rs
 M crates/optix/src/accel.rs
 M crates/optix/src/optix.rs
 M crates/optix/tests/gpu_features.rs
 M docs/src/content/docs/crates/nv-demo.md
?? 0/
```
(plus this report)

## Docs
640 -> 648 B with the new offsets in root `AGENTS.md`, `crates/nv-demo/AGENTS.md` (also: staging growth, classic-motion SDG convention, new traps for `LaunchUpload`/`FractalLaunch`, sky `env` pointer, dock layout, anim ranges), `README.md`, `DIAGRAMS.md`, `docs/src/content/docs/crates/nv-demo.md`, `persist.rs` module doc. `TODO.md` / `plan2.md` not ticked (lead ticks after review).

## Risks / not done
- Visual behaviour not verified: classic object motion (DLSS/temporal denoiser with auto-rotate / sat orbit), a parameterised-to-parameterised sky crossfade, SDG occlusion against the fractal. Needs an interactive look.
- Classic motion is identity on the first frame after the classic instance list changes (sat count, toggles): no history exists then.
- `VULKAN_SUBMIT_TIMEOUT` is a fixed 5 s; on timeout raw Vulkan objects referenced by the commands may still be in use (documented; caller must treat the device as lost). `platform::wait_for_submission` (Drop paths) still waits without a timeout, and `main.rs` `device.poll(wait_indefinitely)` (APP:F27) is unchanged: both out of this item's scope.
- Changing `DockTab` variants in future makes old saved layouts a load problem once (file protected, status line message) - documented as a trap.
- Animating `exposure` now oscillates over the full 0.05..8 range (10 % = 0.795 instead of 0.175).
- Pre-existing doc drift not touched: README / docs site describe the CSG host staging as `256 KiB` (it is 2 MiB growing to 16 MiB, RND:H12).
- Repository is not rustfmt-clean (pre-existing, many files); `cargo fmt` was not run to avoid unrelated churn.
- Not done from TODO's same line: GPU:D07/D08 (optix.h hash in PTX cache key, shared OptiX discovery) were not in this task's list.
